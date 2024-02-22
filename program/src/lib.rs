use iced::{executor, Application, Command, Theme};

pub struct Program {}

impl Application for Program {
    type Executor = executor::Default;
    type Message = ();
    type Theme = Theme;
    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (Program {}, Command::none())
    }

    fn title(&self) -> String {
        "yomisama".to_owned()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        "Hello, world!".into()
    }
}
