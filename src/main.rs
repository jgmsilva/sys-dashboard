use iced::Container;
///use iced::widget::{button, column, row};
use iced::{button, executor, Application, Button, Column, Command, Element, Row, Settings, Text};
use std::fmt;
use std::process::Command as Com;
use std::process::Stdio;
///use strum::IntoEnumIterator; // 0.17.1
///use strum_macros::EnumIter; // 0.17.1

pub fn main() -> iced::Result {
    Dashboard::run(Settings::default())
}

struct Dashboard {
    button_sys: button::State,
    button_proc: button::State,
    button_fil: button::State,
    button_mem: button::State,
    button_term: button::State,
    status: String,
}

#[derive(Debug, Clone, Copy)]
enum Panel {
    System,
    Process,
    Files,
    Memory,
    Terminal,
}

impl fmt::Display for Panel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Panel::System => write!(f, "System"),
            Panel::Process => write!(f, "Process"),
            Panel::Files => write!(f, "Files"),
            Panel::Memory => write!(f, "Memory"),
            Panel::Terminal => write!(f, "Terminal"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Show(Panel),
}
impl Application for Dashboard {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Dashboard, Command<Self::Message>) {
        (
            Dashboard {
                button_sys: button::State::new(),
                button_proc: button::State::new(),
                button_fil: button::State::new(),
                button_mem: button::State::new(),
                button_term: button::State::new(),
                status: Panel::System.to_string(),
            },
            Command::none(),
        )
    }
    fn title(&self) -> String {
        String::from("System Dashboard")
    }

    // protters-iced para fazer os graficos
    //
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Show(panel) => match panel {
                Panel::System => {
                    self.status = String::from_utf8_lossy(
                        &Com::new("uname").arg("-a").output().expect("fail").stdout,
                    )
                    .to_string();
                }
                Panel::Process => {
                    let cmd = Com::new("ps").arg("aux").output().expect("fail").stdout;
                    self.status = String::from_utf8_lossy(&cmd).to_string();
                }
                Panel::Files => {
                    self.status = Panel::Files.to_string();
                }
                Panel::Memory => {
                    self.status = String::from_utf8_lossy(
                        &Com::new("cat")
                            .arg("/proc/meminfo")
                            .output()
                            .expect("fail")
                            .stdout,
                    )
                    .to_string();
                }
                Panel::Terminal => {}
            },
        };
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        /*for pan in Panel::iter() {
            row.push(
                Button::new(&mut self.button, Text::new(pan.to_string()))
                    .on_press(Message::Show(pan)),
            );
        }*/
        Column::new()
            .push(
                Row::new()
                    .push(
                        Button::new(&mut self.button_sys, Text::new(Panel::System.to_string()))
                            .on_press(Message::Show(Panel::System)),
                    )
                    .push(
                        Button::new(&mut self.button_proc, Text::new(Panel::Process.to_string()))
                            .on_press(Message::Show(Panel::Process)),
                    )
                    .push(
                        Button::new(&mut self.button_fil, Text::new(Panel::Files.to_string()))
                            .on_press(Message::Show(Panel::Files)),
                    )
                    .push(
                        Button::new(&mut self.button_mem, Text::new(Panel::Memory.to_string()))
                            .on_press(Message::Show(Panel::Memory)),
                    )
                    .push(
                        Button::new(
                            &mut self.button_term,
                            Text::new(Panel::Terminal.to_string()),
                        )
                        .on_press(Message::Show(Panel::Terminal)),
                    ),
            )
            .push(Text::new(&self.status))
            .into()
    }
}
