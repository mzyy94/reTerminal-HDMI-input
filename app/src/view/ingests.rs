use iced::{
    alignment, button, scrollable, Button, Column, Command, Container, Element, Length, Scrollable,
    Subscription, Text,
};

use crate::{Message, View};

#[derive(Default)]
pub struct App {
    scroll: scrollable::State,
    ingest_buttons: Vec<(crate::ingest::Ingest, button::State)>,
}

impl App {
    pub fn new() -> Self {
        App::default()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::FetchIngest(twitch) => {
                if let Some(twitch) = twitch {
                    self.ingest_buttons = twitch
                        .ingests
                        .iter()
                        .map(|ingest| (ingest.clone(), button::State::new()))
                        .collect();
                }
            }
            Message::SelectIngest(url) => {
                let mut setting = crate::SETTINGS.write().unwrap();
                (*setting).rtmp_url = url;
                return Command::perform(async { View::Setting }, Message::ChangeView);
            }
            _ => {}
        }
        Command::none()
    }

    pub fn view(&mut self) -> Element<Message> {
        let title = Text::new("Ingest List")
            .size(40)
            .horizontal_alignment(alignment::Horizontal::Center)
            .width(Length::Fill);

        let content = self.ingest_buttons.iter_mut().fold(
            Column::new()
                .spacing(10)
                .padding(20)
                .align_items(alignment::Alignment::Center)
                .width(Length::Units(800)),
            |column, (ingest, state)| {
                let content = Column::new()
                    .push(Text::new(ingest.name.clone()).size(30))
                    .push(Text::new(ingest.url_template.clone()));

                let button = Button::new(state, content)
                    .padding(10)
                    .width(Length::Fill)
                    .height(Length::Units(80))
                    .on_press(Message::SelectIngest(ingest.url_template.clone()));

                column.push(button)
            },
        );

        let content: Element<_> = Scrollable::new(&mut self.scroll)
            .width(Length::Fill)
            .align_items(alignment::Alignment::Center)
            .spacing(10)
            .push(title)
            .push(content)
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
