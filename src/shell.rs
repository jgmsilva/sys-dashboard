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
            log: "testing log".to_string(),
            dir: PathBuf::from(env::var("HOME").unwrap()),
        }
    }

    pub fn print(&self) -> &str {
        println!("{}", self.log);
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
                    // default to '/' as new directory if one was not provided
                    let curr_dir = args.peekable().peek().map_or("/home/joao", |x| *x);
                    if curr_dir == ".." {
                        self.dir.pop();
                    } else {
                        let root = Path::new(curr_dir);
                        println!("{}", root.to_str().unwrap());
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

                    let stdout = if commands.peek().is_some() {
                        // there is another command piped behind this one
                        // prepare to send output to the next command
                        Stdio::piped()
                    } else {
                        // there are no more commands piped behind this one
                        // send output to shell stdout
                        Stdio::piped()
                    };

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
            // block until the final command has finished
            out = String::from_utf8(final_command.wait_with_output().unwrap().stdout).unwrap();
        } else {
            out = String::from("");
        }
        println!("out: {}", out);
        self.log += &out;
    }
}
