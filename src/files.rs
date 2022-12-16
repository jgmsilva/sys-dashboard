use std::{path::Path, process::Command};

pub fn make_dirs(path: &Path) -> Vec<String> {
    let dirs = String::from_utf8(
        Command::new("ls")
            .current_dir(path)
            .arg("-a")
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap();
    let lines = dirs.lines();
    let vec: Vec<String> = lines.map(|line| line.to_string()).collect();
    for v in vec.iter() {
        println!("{}", v)
    }
    return vec;
}

pub fn get_disk_space() -> (i32, i32, i32) {
    let cmd = Command::new("df").args(["-ht", "ext4"]).output().unwrap();

    let content = String::from_utf8(cmd.stdout).unwrap();
    let mut lines = content.lines();
    let l = lines.nth(1).unwrap();
    let cols = l.split_whitespace().collect::<Vec<&str>>();
    let mut total = cols[1].chars();
    total.next_back();
    let mut used = cols[2].chars();
    used.next_back();
    let mut perc = cols[4].chars();
    perc.next_back();
    (
        total.as_str().parse::<i32>().unwrap(),
        used.as_str().parse::<i32>().unwrap(),
        perc.as_str().parse::<i32>().unwrap(),
    )
}
