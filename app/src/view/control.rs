use iced::{
    alignment, button, time, Alignment, Color, Column, Command, Container, Element, Image, Length,
    Row, Space, Subscription, Text,
};
use iced_native::{keyboard, subscription, Event};

use anyhow::Error;

use std::time::{Duration, Instant};

use crate::font;
use crate::stream;
use crate::style;
use crate::widget::{action, meter};
use crate::{Message, View};

#[derive(Default)]
pub struct App {
    streamer: stream::Stream,
    start: Option<Instant>,
    voice_off: button::State,
    camera_off: button::State,
    sound_off: button::State,
    video_off: button::State,
    stream_off: button::State,
    first_action: button::State,
    second_action: button::State,
    third_action: button::State,
    broadcast_action: button::State,
    settings: button::State,
}

impl App {
    pub fn new() -> App {
        let mut streamer = stream::Stream::new();
        streamer.create_videopipeline().unwrap();
        streamer.create_audiopipeline().unwrap();
        streamer.run_loop().unwrap();

        App {
            streamer,
            ..App::default()
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            time::every(Duration::from_millis(1000 / 30)).map(Message::UpdateFrame),
            subscription::events().map(Message::Event),
        ])
    }

    fn url_host(&self) -> String {
        let url: &str = &crate::SETTINGS.read().unwrap().rtmp_url;
        let v: Vec<_> = url.split('/').collect();
        if v.len() > 3 {
            v[2].to_string()
        } else {
            "Invalid host".to_string()
        }
    }

    pub fn start_stream(&mut self) -> Result<(), Error> {
        let setting = crate::SETTINGS.read().unwrap();
        let server_url = setting.rtmp_url.clone();
        let stream_key = setting.stream_key.clone();

        let stream_url = &format!(
            "{}{stream_key}",
            server_url.replace("{stream_key}", ""),
            stream_key = stream_key
        );
        self.streamer.start_rtmp(stream_url)?;
        self.start = Some(Instant::now());
        Ok(())
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::UpdateFrame(_) => {}
            Message::Event(event) => {
                if let Event::Keyboard(keyboard::Event::KeyReleased { key_code, .. }) = event {
                    match key_code {
                        keyboard::KeyCode::A => {
                            self.streamer.toggle_camera().unwrap();
                        }
                        keyboard::KeyCode::S => {
                            self.streamer.toggle_mic().unwrap();
                        }
                        keyboard::KeyCode::F => {
                            return Command::perform(async {}, Message::StartStream);
                        }
                        _ => {
                            // TODO: Implement button actions [a/s/d/f]
                            dbg!(key_code);
                        }
                    }
                }
            }
            _ => {}
        }

        Command::none()
    }

    pub fn view(&mut self) -> Element<Message> {
        let url = self.url_host();
        let duration = match self.start {
            Some(start) => start.elapsed(),
            None => Duration::from_secs(0),
        };
        let time = format!(
            "{:02}:{:02}:{:02}",
            duration.as_secs() / 3600,
            (duration.as_secs() / 60) % 60,
            duration.as_secs() % 60,
        );
        let frame = (*self.streamer.get_frame()).clone();
        let image = Image::new(frame)
            .width(Length::Units(1024))
            .height(Length::Units(576));

        let levels = self.streamer.get_levels();

        let meters: Element<_> = Container::new(
            Row::new()
                .width(Length::Fill)
                .spacing(12)
                .align_items(Alignment::Start)
                .push(meter::LevelMeter::new(levels.0).height(Length::Units(576 - 12)))
                .push(meter::LevelMeter::new(levels.1).height(Length::Units(576 - 12))),
        )
        .padding(12)
        .center_x()
        .style(style::MeterArea)
        .into();

        let meter_area = Container::new(meters).padding(12).center_x();

        let left_content = Column::new()
            .align_items(Alignment::Center)
            .push(meter_area)
            .push(
                action::icon(
                    &mut self.settings,
                    font::Icon::Gear,
                    action::IconButton::Round,
                )
                .on_press(Message::ChangeView(View::Setting)),
            );

        let video_area = Container::new(image)
            .padding(12)
            .center_x()
            .style(style::PreviewArea);

        let side_actions: Element<_> = Column::new()
            .width(Length::Fill)
            .spacing(12)
            .align_items(Alignment::End)
            .push(Space::with_height(Length::Units(104)))
            .push(action::icon(
                &mut self.voice_off,
                font::Icon::MicrophoneSlash,
                action::IconButton::Active,
            ))
            .push(action::icon(
                &mut self.camera_off,
                font::Icon::VideoSlash,
                action::IconButton::Inactive,
            ))
            .push(action::icon(
                &mut self.sound_off,
                font::Icon::VolumeXMark,
                action::IconButton::Inactive,
            ))
            .push(action::icon(
                &mut self.video_off,
                font::Icon::Clapperboard,
                action::IconButton::Inactive,
            ))
            .push(action::icon(
                &mut self.stream_off,
                font::Icon::CirclePause,
                action::IconButton::Inactive,
            ))
            .into();

        let main_content: Element<_> = Row::new()
            .width(Length::Fill)
            .align_items(Alignment::Start)
            .push(video_area)
            .push(side_actions)
            .into();

        let text = |text: &str| -> Text {
            Text::new(text)
                .size(40)
                .height(Length::Units(40))
                .font(font::PLEXMONO)
                .color(Color::WHITE)
                .vertical_alignment(alignment::Vertical::Center)
                .horizontal_alignment(alignment::Horizontal::Right)
        };

        let icon = |icon: font::Icon| -> Text {
            Text::new(icon)
                .size(40)
                .font(font::ICONS)
                .color(Color::WHITE)
                .vertical_alignment(alignment::Vertical::Center)
                .horizontal_alignment(alignment::Horizontal::Center)
        };

        let status_area: Element<_> = Row::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(Alignment::Center)
            .spacing(10)
            .push(icon(font::Icon::Microchip))
            .push(text("100%").width(Length::Units(90)))
            .push(icon(font::Icon::Gauge))
            .push(text("1000 ms").width(Length::Units(150)))
            .push(icon(font::Icon::Stopwatch))
            .push(text(&time).width(Length::Units(150)))
            .push(icon(font::Icon::CloudArrowUp))
            .push(text(&url).horizontal_alignment(alignment::Horizontal::Left))
            .into();

        let bottom_actions: Element<_> = Row::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(Alignment::End)
            .spacing(4)
            .push(Space::with_width(Length::Fill))
            .push(action::text(
                &mut self.first_action,
                "CAMERA",
                if self.streamer.camera_off() {
                    action::LabelButton::Inactive
                } else {
                    action::LabelButton::Active
                },
            ))
            .push(action::text(
                &mut self.second_action,
                "MIC",
                if self.streamer.mic_off() {
                    action::LabelButton::Inactive
                } else {
                    action::LabelButton::Active
                },
            ))
            .push(action::text(
                &mut self.third_action,
                "SCENE",
                action::LabelButton::Action,
            ))
            .push(action::text(
                &mut self.broadcast_action,
                "START",
                action::LabelButton::Primary,
            ))
            .into();

        let right_content: Element<_> = Column::new()
            .height(Length::Fill)
            .spacing(2)
            .push(main_content)
            .push(status_area)
            .push(bottom_actions)
            .into();

        let content: Element<_> = Row::new()
            .height(Length::Fill)
            .push(left_content)
            .push(right_content)
            .into();

        #[cfg(feature = "debug")]
        let content = content.explain(iced::Color::BLACK);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(0)
            .center_x()
            .center_y()
            .style(style::Container)
            .into()
    }
}
