use iced::{executor, Application, Command, Element, Settings, Text};

pub fn main() -> iced::Result {
    Dashboard::run(Settings::default())
}

struct Dashboard;

impl Application for Dashboard {
    type Executor = executor::Default;
    type Message = ();
    type Flags = ();

    fn new(_flags: ()) -> (Dashboard, Command<Self::Message>) {
        (Dashboard, Command::none())
    }
    fn title(&self) -> String {
        String::from("System Dashboard")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        Column
    }
}
