use iced::{
    alignment, button, pick_list, text_input, Button, Checkbox, Column, Command, Container,
    Element, Length, PickList, Space, Subscription, Text, TextInput,
};

use crate::ingest::Service;
use crate::View;

#[derive(Debug, Clone)]
pub enum Message {
    ToggleSecureInput(bool),
    InputChanged(String),
    SelectIngestService(Service),
    UpdateSetting,
}

#[derive(Default)]
pub struct App {
    back: button::State,
    select_service: pick_list::State<Service>,
    input_url: text_input::State,
    input_key: text_input::State,
    ingest_service: Option<Service>,
    custom_url: String,
    stream_key: String,
    is_secure: bool,
}

impl super::ViewApp for App {
    type LocalMessage = Message;

    fn new() -> Self {
        let mut app = App {
            is_secure: true,
            ..App::default()
        };
        app.refresh();
        app
    }

    fn subscription(&self) -> Subscription<Self::LocalMessage> {
        Subscription::none()
    }

    fn update(&mut self, message: Self::LocalMessage) -> Command<crate::Message> {
        match message {
            Message::ToggleSecureInput(is_secure) => {
                self.is_secure = is_secure;
            }
            Message::InputChanged(changed) => {
                if self.input_url.is_focused() {
                    self.custom_url = changed;
                } else if self.input_key.is_focused() {
                    self.stream_key = changed;
                }
            }
            Message::UpdateSetting => {
                let mut setting = crate::SETTINGS.write().unwrap();
                (*setting).ingest_service = self.ingest_service.clone();
                (*setting).rtmp_url = self.custom_url.clone();
                (*setting).stream_key = self.stream_key.clone();

                return Command::perform(async { View::Control }, crate::Message::ChangeView);
            }
            Message::SelectIngestService(ingest) => {
                self.ingest_service = Some(ingest);
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<crate::Message> {
        let title = Text::new("Setting")
            .size(40)
            .horizontal_alignment(alignment::Horizontal::Center)
            .width(Length::Fill);

        let url_label = Text::new("RTMP Server")
            .size(20)
            .horizontal_alignment(alignment::Horizontal::Left)
            .width(Length::Fill);

        let select_service = PickList::new(
            &mut self.select_service,
            &Service::ALL[..],
            self.ingest_service,
            |event| Message::SelectIngestService(event).into(),
        )
        .placeholder("Choose Streaming Service...")
        .padding(10)
        .width(Length::Fill);

        let url_input: Element<_> = match self.ingest_service {
            Some(Service::Custom) => TextInput::new(
                &mut self.input_url,
                "rtmp://live.example.com:1935/live/{stream_key}",
                &self.custom_url,
                |event| Message::InputChanged(event).into(),
            )
            .padding(10)
            .size(30)
            .into(),
            _ => Space::with_height(Length::Units(1)).into(),
        };

        let key_label = Text::new("Stream Key")
            .size(20)
            .horizontal_alignment(alignment::Horizontal::Left)
            .width(Length::Fill);

        let key_input = TextInput::new(
            &mut self.input_key,
            "0a1b2c3d4e5f",
            &self.stream_key,
            |event| Message::InputChanged(event).into(),
        )
        .padding(10)
        .size(30);

        let key_input = if self.is_secure {
            key_input.password()
        } else {
            key_input
        };

        let checkbox = Checkbox::new(self.is_secure, "Hide Stream Key", |event| {
            Message::ToggleSecureInput(event).into()
        })
        .width(Length::Fill);

        let save_button = Button::new(&mut self.back, Text::new("Save"))
            .padding(10)
            .on_press(Message::UpdateSetting.into());

        let content: Element<_> = Column::new()
            .spacing(20)
            .padding(20)
            .align_items(alignment::Alignment::Center)
            .width(Length::Units(800))
            .push(title)
            .push(url_label)
            .push(select_service)
            .push(url_input)
            .push(key_label)
            .push(key_input)
            .push(checkbox)
            .push(save_button)
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

impl App {
    pub fn refresh(&mut self) -> () {
        let setting = crate::SETTINGS.read().unwrap();
        self.custom_url = setting.rtmp_url.clone();
        self.stream_key = setting.stream_key.clone();
        self.ingest_service = setting.ingest_service;
    }
}
