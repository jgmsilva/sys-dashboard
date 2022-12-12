use iced::text_input::{self, TextInput};
use iced::{
    button, executor, scrollable, Application, Button, Column, Command, Container, Element, Length,
    ProgressBar, Row, Scrollable, Settings, Text,
};
use panel::*;
use process::process_container;
use shell::*;
use std::env;
use std::path::{Path, PathBuf};
///use iced::widget::{button, column, row};
use std::process::Command as Comm;
mod files;
mod memory;
mod panel;
mod process;
mod shell;
mod system;
///use strum::IntoEnumIterator; // 0.17.1
///use strum_macros::EnumIter; // 0.17.1

pub fn main() -> iced::Result {
    Dashboard::run(Settings::default())
}

struct Dashboard {
    buttons: Vec<(String, button::State)>,
    button_sys: button::State,
    button_proc: button::State,
    button_fil: button::State,
    button_mem: button::State,
    button_term: button::State,
    status: Panel,
    shell: Shell,
    scroll: scrollable::State,
    input: text_input::State,
    input_value: String,
    file_dir: PathBuf,
}

#[derive(Debug, Clone)]
pub enum Message {
    Show(Panel),
    InputChanged(String),
    ExecuteCommand,
    ChangeDir(String),
}
impl Application for Dashboard {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Dashboard, Command<Self::Message>) {
        if let Err(e) = env::set_current_dir(env::var("HOME").unwrap()) {
            println!("erro ao inicializar: {}", e);
        }
        (
            Dashboard {
                buttons: Vec::<(String, button::State)>::new(),
                button_sys: button::State::new(),
                button_proc: button::State::new(),
                button_fil: button::State::new(),
                button_mem: button::State::new(),
                button_term: button::State::new(),
                status: Panel::System,
                shell: Shell::new(),
                scroll: scrollable::State::new(),
                input: text_input::State::new(),
                input_value: String::new(),
                file_dir: PathBuf::from("/home/joao"),
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
                Panel::Files => {
                    self.update(Message::ChangeDir(
                        self.file_dir.to_string_lossy().into_owned(),
                    ));
                    self.status = panel;
                }
                panel => self.status = panel,
            },
            Message::InputChanged(strig) => self.input_value = strig,
            Message::ExecuteCommand => {
                self.shell.exec(&self.input_value);
                self.input_value = String::new();
                self.scroll.snap_to(1.0);
            }
            Message::ChangeDir(path) => {
                self.buttons.clear();
                if path == ".." {
                    self.file_dir.pop();
                } else {
                    let root = Path::new(&path);
                    self.file_dir.push(root);
                }
                for line in files::make_dirs(self.file_dir.as_path()) {
                    self.buttons.push((line.clone(), button::State::new()))
                }
            }
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
        let (used, total) = memory::get_mem();
        Column::new()
            .push(
                Row::new()
                    .push(
                        Button::new(&mut self.button_sys, Text::new(Panel::System.to_string()))
                            .on_press(Message::Show(Panel::System))
                            .width(Length::Units(150)),
                    )
                    .push(
                        Button::new(&mut self.button_proc, Text::new(Panel::Process.to_string()))
                            .on_press(Message::Show(Panel::Process))
                            .width(Length::Units(150)),
                    )
                    .push(
                        Button::new(&mut self.button_fil, Text::new(Panel::Files.to_string()))
                            .on_press(Message::Show(Panel::Files))
                            .width(Length::Units(150)),
                    )
                    .push(
                        Button::new(&mut self.button_mem, Text::new(Panel::Memory.to_string()))
                            .on_press(Message::Show(Panel::Memory))
                            .width(Length::Units(150)),
                    )
                    .push(
                        Button::new(
                            &mut self.button_term,
                            Text::new(Panel::Terminal.to_string()),
                        )
                        .on_press(Message::Show(Panel::Terminal))
                        .width(Length::Units(150)),
                    )
                    .spacing(15)
                    .width(Length::Fill),
            )
            .push(match self.status {
                Panel::System => Container::new(
                    Column::new()
                        .push(Text::new(format!("Kernel {}", system::get_kernel())))
                        .push(Text::new(format!("Cpu {}", system::get_cpu()))),
                ),
                Panel::Memory => Container::new(
                    Column::new()
                        .push(Text::new("Memory Used:"))
                        .push(ProgressBar::new(0.0..=total, used)),
                ),
                Panel::Files => Container::new(
                    Scrollable::new(&mut self.scroll)
                        .push(self.buttons.iter_mut().enumerate().fold(
                            Column::new(),
                            |column, (i, (line, state))| {
                                column.push(
                                    Button::new(state, Text::new(line.as_str()))
                                        .on_press(Message::ChangeDir(line.clone()))
                                        .width(Length::Units(200)),
                                )
                            },
                        ))
                        .width(Length::Fill),
                ),
                Panel::Process => Container::new(process_container()),
                Panel::Terminal => Container::new(
                    Column::new()
                        .push(
                            Scrollable::new(&mut self.scroll)
                                .push(Text::new(self.shell.print()))
                                .height(Length::Fill)
                                .width(Length::Fill),
                        )
                        .push(
                            Column::new()
                                .push(
                                    Text::new(self.shell.current_dir())
                                        .color([0.0, 160.0 / 255.0, 0.0])
                                        .size(40)
                                        .width(Length::Fill),
                                )
                                .push(
                                    TextInput::new(
                                        &mut self.input,
                                        "",
                                        &mut self.input_value,
                                        Message::InputChanged,
                                    )
                                    .on_submit(Message::ExecuteCommand),
                                ),
                        ),
                ),
            })
            .into()
    }
}
