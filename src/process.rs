use iced::{Column, Container, Text};
use std::process::Command;
pub fn process_container() -> Text {
    let cmd = Command::new("ps").arg("aux").output().unwrap();
    let result = String::from_utf8(cmd.stdout).unwrap();
    Text::new(result)
}
