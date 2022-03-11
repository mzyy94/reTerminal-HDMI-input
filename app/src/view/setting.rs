use iced::{
    alignment, button, text_input, Button, Checkbox, Column, Command, Container, Element, Length,
    Subscription, Text, TextInput,
};

use crate::{Message, View};

#[derive(Default)]
pub struct App {
    back: button::State,
    input_url: text_input::State,
    input_key: text_input::State,
    server_url: String,
    stream_key: String,
    is_secure: bool,
}

impl App {
    pub fn new() -> Self {
        App {
            is_secure: true,
            ..App::default()
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ToggleSecureInput(is_secure) => {
                self.is_secure = is_secure;
            }
            Message::InputChanged(changed) => {
                if self.input_url.is_focused() {
                    self.server_url = changed;
                } else if self.input_key.is_focused() {
                    self.stream_key = changed;
                }
            }
            _ => {}
        }
        Command::none()
    }

    pub fn view(&mut self) -> Element<Message> {
        let title = Text::new("Setting")
            .size(40)
            .horizontal_alignment(alignment::Horizontal::Center)
            .width(Length::Fill);

        let url_label = Text::new("RTMP Server URL")
            .size(20)
            .horizontal_alignment(alignment::Horizontal::Left)
            .width(Length::Fill);

        let url_input = TextInput::new(
            &mut self.input_url,
            "rtmp://live.example.com:1935/live/",
            &self.server_url,
            Message::InputChanged,
        )
        .padding(10)
        .size(30);

        let key_label = Text::new("Streaming Key")
            .size(20)
            .horizontal_alignment(alignment::Horizontal::Left)
            .width(Length::Fill);

        let key_input = TextInput::new(
            &mut self.input_key,
            "0a1b2c3d4e5f",
            &self.stream_key,
            Message::InputChanged,
        )
        .padding(10)
        .size(30);

        let key_input = if self.is_secure {
            key_input.password()
        } else {
            key_input
        };

        let checkbox = Checkbox::new(
            self.is_secure,
            "Hide streaming key",
            Message::ToggleSecureInput,
        )
        .width(Length::Fill);

        let button = Button::new(&mut self.back, Text::new("Save"))
            .padding(10)
            .on_press(Message::ChangeView(View::Control));

        let content: Element<_> = Column::new()
            .spacing(20)
            .padding(20)
            .align_items(alignment::Alignment::Center)
            .width(Length::Units(800))
            .push(title)
            .push(url_label)
            .push(url_input)
            .push(key_label)
            .push(key_input)
            .push(checkbox)
            .push(button)
            .into();

        #[cfg(feature = "debug")]
        let content = content.explain(iced::Color::BLACK);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(0)
            .center_x()
            .into()
    }
}
