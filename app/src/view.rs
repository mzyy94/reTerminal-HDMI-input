use iced::{Command, Element, Subscription};

pub mod control;
pub mod ingests;
pub mod setting;

pub trait ViewApp {
    fn new() -> Self;
    fn subscription(&self) -> Subscription<crate::Message>;
    fn update(&mut self, message: crate::Message) -> Command<crate::Message>;
    fn view(&mut self) -> Element<crate::Message>;
}

#[derive(Debug, Clone)]
pub enum View {
    Control,
    Setting,
    Ingests,
}
