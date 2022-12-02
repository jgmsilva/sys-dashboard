///use iced::widget::{button, column, row};
use iced::{button, executor, Application, Button, Column, Command, Element, Row, Settings, Text};
use std::fmt;
///use strum::IntoEnumIterator; // 0.17.1
///use strum_macros::EnumIter; // 0.17.1

pub fn main() -> iced::Result {
    Dashboard::run(Settings::default())
}

struct Dashboard {
    buttonSys: button::State,
    buttonProc: button::State,
    buttonFil: button::State,
    buttonMem: button::State,
    buttonTerm: button::State,
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
                buttonSys: button::State::new(),
                buttonProc: button::State::new(),
                buttonFil: button::State::new(),
                buttonMem: button::State::new(),
                buttonTerm: button::State::new(),
                status: Panel::System.to_string(),
            },
            Command::none(),
        )
    }
    fn title(&self) -> String {
        String::from("System Dashboard")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Show(panel) => match panel {
                Panel::System => {
                    self.status = Panel::System.to_string();
                }
                Panel::Process => {
                    self.status = Panel::Process.to_string();
                }
                Panel::Files => {
                    self.status = Panel::Files.to_string();
                }
                Panel::Memory => {
                    self.status = Panel::Memory.to_string();
                }
                Panel::Terminal => {
                    self.status = Panel::Terminal.to_string();
                }
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
                        Button::new(&mut self.buttonSys, Text::new(Panel::System.to_string()))
                            .on_press(Message::Show(Panel::System)),
                    )
                    .push(
                        Button::new(&mut self.buttonProc, Text::new(Panel::Process.to_string()))
                            .on_press(Message::Show(Panel::Process)),
                    )
                    .push(
                        Button::new(&mut self.buttonFil, Text::new(Panel::Files.to_string()))
                            .on_press(Message::Show(Panel::Files)),
                    )
                    .push(
                        Button::new(&mut self.buttonMem, Text::new(Panel::Memory.to_string()))
                            .on_press(Message::Show(Panel::Memory)),
                    )
                    .push(
                        Button::new(&mut self.buttonTerm, Text::new(Panel::Terminal.to_string()))
                            .on_press(Message::Show(Panel::Terminal)),
                    ),
            )
            .push(Text::new(&self.status))
            .into()
    }
}
