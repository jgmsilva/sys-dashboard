use std::env;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};

pub struct Shell {
    log: String,
    dir: PathBuf,
}

impl Shell {
    pub fn new() -> Self {
        Shell {
            log: "".to_string(),
            dir: PathBuf::from(env::var("HOME").unwrap()),
        }
    }

    pub fn print(&self) -> &str {
        &self.log
    }

    pub fn current_dir(&self) -> &str {
        self.dir.to_str().unwrap()
    }

    pub fn exec(&mut self, input: &str) {
        let mut commands = input.trim().split(" | ").peekable();
        let mut previous_command = None;

        while let Some(comm) = commands.next() {
            let mut parts = comm.trim().split_whitespace();
            let command = parts.next().unwrap();
            let args = parts;

            match command {
                "cd" => {
                    let home = env::var("HOME").unwrap();
                    let curr_dir = args.peekable().peek().map_or(home.as_str(), |x| *x);
                    if curr_dir == ".." {
                        self.dir.pop();
                    } else {
                        let root = Path::new(curr_dir);
                        self.dir.push(root);
                    }
                    previous_command = None;
                }
                "clear" => self.log = String::new(),
                "exit" => return,
                command => {
                    let stdin = previous_command.map_or(Stdio::inherit(), |output: Child| {
                        Stdio::from(output.stdout.unwrap())
                    });

                    let stdout = Stdio::piped();

                    let output = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .current_dir(self.dir.as_path())
                        .spawn();

                    match output {
                        Ok(output) => {
                            previous_command = Some(output);
                        }
                        Err(e) => {
                            previous_command = None;
                            eprintln!("{}", e);
                        }
                    };
                }
            }
        }
        let out;
        if let Some(final_command) = previous_command {
            out = String::from_utf8(final_command.wait_with_output().unwrap().stdout).unwrap();
        } else {
            out = String::from("");
        }
        self.log += &out;
    }
}
