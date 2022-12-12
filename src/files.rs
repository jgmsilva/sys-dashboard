use iced::{button, Button, Column, Text};
use std::{path::Path, process::Command};

use crate::Message;

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
