use crate::Message;
use iced::{Column, Element, Row, Rule, Text, Length};
use std::process::{Command, Stdio};

struct Process {
    owner: String,
    pid: u32,
    pcpu: f32,
    pmem: f32,
    time: String,
    command: String,
}
pub fn process_container() -> Element<'static, Message> {
    let cmd = Command::new("ps")
        .args(["aux", "--sort=-pcpu"])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let limit = Command::new("head")
        .args(["-n", "15"])
        .stdin(cmd.stdout.unwrap())
        .output()
        .unwrap();
    let result = String::from_utf8(limit.stdout).unwrap();
    let lines = result.lines();

    let mut row: Column<Message> = Column::new().width(iced::Length::Shrink);
    for line in lines {
        let content = line.split_whitespace().skip(10).fold(String::new(), |acc, s| acc + " " + s);
        let mut rcols: Vec<String> = line.split_whitespace().take(10).map(|s| String::from(s)).collect::<Vec<String>>();
        rcols.push(content);
        let mut cont: Row<Message> = Row::new()
            .height(iced::Length::Units(30))
            .width(iced::Length::Shrink);
        for (i, col) in rcols.iter().enumerate() {
            cont = cont.push(Rule::vertical(15)).push(Text::new(String::from(col)).width(if i == 10 {Length::Fill}else{Length::Units(60)}));
        }
        row = row.push(cont).push(Rule::horizontal(10));
    }
    row.into()
}
