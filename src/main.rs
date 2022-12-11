use iced::text_input::{self, TextInput};
use iced::{
    button, executor, Application, Button, Column, Command, Container, Element, Row, Settings, Text,
};
use panel::*;
use shell::*;
use std::env;
///use iced::widget::{button, column, row};
use std::process::Command as Comm;
mod panel;
mod shell;
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
    status: Panel,
    shell: Shell,
    input: text_input::State,
    input_value: String,
}

#[derive(Debug, Clone)]
enum Message {
    Show(Panel),
    InputChanged(String),
    CreateTask,
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
                status: Panel::System,
                shell: Shell::new(),
                input: text_input::State::new(),
                input_value: String::new(),
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
            Message::Show(panel) => self.status = panel,
            Message::InputChanged(strig) => self.input_value = strig,
            Message::CreateTask => {
                self.shell.exec(&self.input_value);
                self.input_value = String::new();
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
            .push(match self.status {
                Panel::System => Container::new(Text::new("system")),
                Panel::Memory => Container::new(Text::new("memory")),
                Panel::Files => Container::new(Text::new(
                    std::str::from_utf8(&Comm::new("ls").output().unwrap().stdout).unwrap(),
                )),
                Panel::Process => Container::new(Text::new("process")),
                Panel::Terminal => Container::new(
                    Column::new().push(Text::new(self.shell.print())).push(
                        Column::new()
                            .push(Text::new(self.shell.current_dir()))
                            .push(
                                TextInput::new(
                                    &mut self.input,
                                    "What needs to be done?",
                                    &mut self.input_value,
                                    Message::InputChanged,
                                )
                                .padding(15)
                                .on_submit(Message::CreateTask),
                            ),
                    ),
                ),
            })
            .into()
    }
}
