use iced::{Command, Element, Subscription};

pub mod control;
pub mod ingests;
pub mod setting;

pub trait ViewApp {
    type LocalMessage;

    fn new() -> Self;
    fn subscription(&self) -> Subscription<Self::LocalMessage>;
    fn update(&mut self, message: Self::LocalMessage) -> Command<crate::Message>;
    fn view(&mut self) -> Element<crate::Message>;
}

#[derive(Debug, Clone)]
pub enum View {
    Control,
    Setting,
    Ingests,
}

#[derive(Debug, Clone)]
pub enum ViewMessage {
    Control(<control::App as ViewApp>::LocalMessage),
    Setting(<setting::App as ViewApp>::LocalMessage),
    Ingests(<ingests::App as ViewApp>::LocalMessage),
}

impl Into<crate::Message> for ViewMessage {
    fn into(self) -> crate::Message {
        crate::Message::ViewMessage(self)
    }
}

impl Into<crate::Message> for <control::App as ViewApp>::LocalMessage {
    fn into(self) -> crate::Message {
        crate::Message::ViewMessage(ViewMessage::Control(self).into())
    }
}

impl Into<crate::Message> for <setting::App as ViewApp>::LocalMessage {
    fn into(self) -> crate::Message {
        crate::Message::ViewMessage(ViewMessage::Setting(self).into())
    }
}

impl Into<crate::Message> for <ingests::App as ViewApp>::LocalMessage {
    fn into(self) -> crate::Message {
        crate::Message::ViewMessage(ViewMessage::Ingests(self).into())
    }
}
