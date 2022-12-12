use iced::{button, Button, Column, Text};
use std::{path::Path, process::Command};

use crate::Message;

pub fn make_dirs(path: &Path) -> std::str::Lines {
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

    for dir in lines {
        println!("{}", dir);
    }
    return lines;
}
