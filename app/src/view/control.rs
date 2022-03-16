use iced::{
    alignment, button, time, Alignment, Color, Column, Command, Container, Element, Image, Length,
    Row, Space, Subscription, Text,
};
use iced_native::{keyboard, subscription, Event};

use anyhow::Error;

use std::time::{Duration, Instant};

use crate::font;
use crate::ingest::{IngestError, Service};
use crate::stream;
use crate::style;
use crate::widget::{action, label, meter};
use crate::View;

#[derive(Debug, Clone)]
pub enum Message {
    Event(Event),
    UpdateFrame(Instant),
    StartStream(Result<String, IngestError>),
}

#[derive(Default)]
pub struct App {
    streamer: stream::Stream,
    rtmp_host: String,
    start: Option<Instant>,
    settings: button::State,
}

impl super::ViewApp for App {
    type LocalMessage = Message;

    fn new() -> App {
        let mut streamer = stream::Stream::new();
        streamer.create_videopipeline().unwrap();
        streamer.create_audiopipeline().unwrap();
        streamer.run_loop().unwrap();

        let mut app = App {
            streamer,
            ..App::default()
        };
        app.reload_setting();
        app
    }

    fn subscription(&self) -> Subscription<Self::LocalMessage> {
        Subscription::batch([
            time::every(Duration::from_millis(1000 / 30)).map(Message::UpdateFrame),
            subscription::events().map(Message::Event),
        ])
    }

    fn view(&mut self) -> Element<crate::Message> {
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

        let output_levels = self.streamer.get_output_levels();

        let meters: Element<_> = Container::new(
            Row::new()
                .width(Length::Fill)
                .spacing(12)
                .align_items(Alignment::Start)
                .push(meter::LevelMeter::new(output_levels.0).height(Length::Units(576 - 12)))
                .push(meter::LevelMeter::new(output_levels.1).height(Length::Units(576 - 12))),
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
                .on_press(crate::Message::ChangeView(View::Setting)),
            );

        let video_area = Container::new(image)
            .padding(12)
            .center_x()
            .style(style::PreviewArea);

        let mic_icon = Text::new(font::Icon::MicrophoneLines)
            .font(font::ICONS)
            .width(Length::Units(36))
            .horizontal_alignment(alignment::Horizontal::Center)
            .vertical_alignment(alignment::Vertical::Center)
            .size(36);

        let mic_levels = if self.streamer.mic_off() {
            &(0.0, 0.0)
        } else {
            self.streamer.get_mic_levels()
        };

        let meters: Element<_> = Container::new(
            Column::new()
                .spacing(16)
                .align_items(Alignment::Center)
                .push(Space::with_height(Length::Units(0)))
                .push(mic_icon)
                .push(
                    Row::new()
                        .spacing(12)
                        .align_items(Alignment::Start)
                        .push(meter::LevelMeter::new(mic_levels.0).height(Length::Units(576 - 80)))
                        .push(meter::LevelMeter::new(mic_levels.1).height(Length::Units(576 - 80))),
                ),
        )
        .padding(12)
        .center_x()
        .style(style::MeterArea)
        .into();

        let meter_area = Container::new(meters).padding(12).center_x();

        let actions: Element<_> = Column::new()
            .width(Length::Fill)
            .spacing(12)
            .align_items(Alignment::End)
            .push(Space::with_height(Length::Units(104)))
            .push(label::icon_text!(
                font::Icon::Microphone,
                "+",
                label::Label::Inactive
            ))
            .push(label::icon_text!(
                font::Icon::Microphone,
                "-",
                label::Label::Inactive
            ))
            .push(label::icon_text!(
                font::Icon::VolumeOff,
                "+",
                label::Label::Inactive
            ))
            .push(label::icon_text!(
                font::Icon::VolumeOff,
                "-",
                label::Label::Inactive
            ))
            .push(label::icon_text!(
                font::Icon::Shuffle,
                label::Label::Inactive
            ))
            .into();

        let side_actions: Element<_> = Row::new()
            .width(Length::Fill)
            .align_items(Alignment::Center)
            .push(meter_area)
            .push(actions)
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
            .push(icon(font::Icon::Stopwatch))
            .push(text(&time).width(Length::Units(150)))
            .push(icon(font::Icon::CloudArrowUp))
            .push(text(&self.rtmp_host).horizontal_alignment(alignment::Horizontal::Left))
            .into();

        let bottom_actions: Element<_> = Row::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(Alignment::End)
            .spacing(4)
            .push(Space::with_width(Length::Fill))
            .push(label::text(
                "CAMERA",
                if self.streamer.camera_off() {
                    label::Label::Inactive
                } else {
                    label::Label::Active
                },
            ))
            .push(label::text(
                "MIC",
                if self.streamer.mic_off() {
                    label::Label::Inactive
                } else {
                    label::Label::Active
                },
            ))
            .push(label::text("SCENE", label::Label::Inactive))
            .push(label::text("START", label::Label::Primary))
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

    fn update(&mut self, message: Self::LocalMessage) -> Command<crate::Message> {
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
                            return Command::perform(
                                Service::get_ingest_url(),
                                Message::StartStream,
                            )
                            .map(|e| e.into());
                        }
                        _ => {
                            // TODO: Implement button actions [a/s/d/f]
                            dbg!(key_code);
                        }
                    }
                }
            }
            Message::StartStream(url) => match url {
                Ok(url) => self.start_stream(url).unwrap(),
                _ => {}
            },
        }

        Command::none()
    }
}

impl App {
    pub fn reload_setting(&mut self) {
        let setting = crate::SETTINGS.read().unwrap();
        let service = setting.broadcast.ingest_service;
        self.rtmp_host = match service {
            Some(crate::ingest::Service::Custom) => {
                let url: &str = &setting.broadcast.custom_url;
                let v: Vec<_> = url.split('/').collect();
                if v.len() > 3 {
                    v[2].to_string()
                } else {
                    "Invalid host".to_string()
                }
            }
            None => "Invalid host".to_string(),
            Some(service) => service.to_string(),
        };
        #[cfg(feature = "button-shim")]
        {
            let mut buttonshim = buttonshim::ButtonShim::new().unwrap();
            if self.rtmp_host != "Invalid host".to_string() {
                buttonshim.led.set_pixel(0, 0xff, 0).unwrap();
            } else {
                buttonshim.led.set_pixel(0xff, 0xff, 0).unwrap();
            }
        }
    }

    pub fn start_stream(&mut self, server_url: String) -> Result<(), Error> {
        let stream_key = {
            let setting = crate::SETTINGS.read().unwrap();
            setting.broadcast.stream_key.clone()
        };

        let stream_url = &format!(
            "{}{stream_key}",
            server_url.replace("{stream_key}", ""),
            stream_key = stream_key
        );
        self.streamer.start_rtmp(stream_url)?;
        self.start = Some(Instant::now());

        #[cfg(feature = "button-shim")]
        {
            let mut buttonshim = buttonshim::ButtonShim::new().unwrap();
            buttonshim.led.set_pixel(0xff, 0, 0).unwrap();
        }

        Ok(())
    }
}
