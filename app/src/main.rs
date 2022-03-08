use iced::{
  alignment, button, executor, time, window, Alignment, Application, Color, Column, Command,
  Container, Element, Image, Length, Row, Settings, Space, Subscription, Text,
};
use iced_native::{keyboard, subscription, Event};

use std::env;
use std::time::{Duration, Instant};

mod action;
mod font;
mod meter;
mod stream;
mod style;

pub fn main() -> iced::Result {
  App::run(Settings {
    antialiasing: true,
    window: window::Settings {
      size: (1280, 720),
      resizable: false,
      ..window::Settings::default()
    },
    ..Settings::default()
  })
}

#[derive(Default)]
struct App {
  streamer: stream::Stream,
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

#[derive(Debug, Clone)]
pub enum Message {
  Event(Event),
  UpdateFrame(Instant),
}

impl Application for App {
  type Executor = executor::Default;
  type Message = Message;
  type Flags = ();

  fn new(_flags: ()) -> (App, Command<Self::Message>) {
    let streamer = stream::Stream::new()
      .create_videopipeline()
      .and_then(|s| s.create_audiopipeline())
      .and_then(|s| s.run_loop())
      .expect("Failed to start pipeline.");
    (
      App {
        streamer,
        ..App::default()
      },
      Command::none(),
    )
  }

  fn title(&self) -> String {
    String::from("broadcast-terminal")
  }

  fn mode(&self) -> window::Mode {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && &args[1] == "--fullscreen" {
      window::Mode::Fullscreen
    } else {
      window::Mode::Windowed
    }
  }

  fn subscription(&self) -> Subscription<Self::Message> {
    Subscription::batch([
      time::every(Duration::from_millis(1000 / 30)).map(Message::UpdateFrame),
      subscription::events().map(Message::Event),
    ])
  }

  fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
    match message {
      Message::UpdateFrame(_) => {}
      Message::Event(event) => {
        if let Event::Keyboard(keyboard::Event::KeyReleased {
          key_code,
          modifiers: _,
        }) = event
        {
          // TODO: Implement button actions [a/s/d/f]
          dbg!(key_code);
        }
      }
    }
    Command::none()
  }

  fn view(&mut self) -> Element<Message> {
    let frame = (*self.streamer.frame_rx.latest()).clone();
    let image = Image::new(frame)
      .width(Length::Units(1024))
      .height(Length::Units(576));

    let meters: Element<_> = Container::new(
      Row::new()
        .width(Length::Fill)
        .spacing(12)
        .align_items(Alignment::Start)
        .push(
          meter::LevelMeter::new(*self.streamer.sound_left_rx.latest())
            .height(Length::Units(576 - 12)),
        )
        .push(
          meter::LevelMeter::new(*self.streamer.sound_left_rx.latest())
            .height(Length::Units(576 - 12)),
        ),
    )
    .padding(12)
    .center_x()
    .style(style::MeterArea)
    .into();

    let meter_area = Container::new(meters).padding(12).center_x();

    let left_content = Column::new()
      .align_items(Alignment::Center)
      .push(meter_area)
      .push(action::icon(
        &mut self.settings,
        font::Icon::Settings,
        action::IconButton::Round,
      ));

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
        font::Icon::MicOff,
        action::IconButton::Active,
      ))
      .push(action::icon(
        &mut self.camera_off,
        font::Icon::VideoCamOff,
        action::IconButton::Inactive,
      ))
      .push(action::icon(
        &mut self.sound_off,
        font::Icon::VolumeOff,
        action::IconButton::Inactive,
      ))
      .push(action::icon(
        &mut self.video_off,
        font::Icon::Movie,
        action::IconButton::Inactive,
      ))
      .push(action::icon(
        &mut self.stream_off,
        font::Icon::PauseCircle,
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
        .font(font::ROBOTOMONO)
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
      .push(icon(font::Icon::DeveloperBoard))
      .push(text("100%").width(Length::Units(90)))
      .push(icon(font::Icon::NetworkCheck))
      .push(text("1000 ms").width(Length::Units(150)))
      .push(icon(font::Icon::AvTimer))
      .push(text("00:00:00").width(Length::Units(150)))
      .push(icon(font::Icon::CloudUpload))
      .push(text("rtmp.stream.example.com").horizontal_alignment(alignment::Horizontal::Left))
      .into();

    let bottom_actions: Element<_> = Row::new()
      .width(Length::Fill)
      .height(Length::Fill)
      .align_items(Alignment::End)
      .spacing(4)
      .push(Space::with_width(Length::Fill))
      .push(action::text(
        &mut self.first_action,
        "LAYOUT",
        action::LabelButton::Action,
      ))
      .push(action::text(
        &mut self.second_action,
        "CHAT",
        action::LabelButton::Action,
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
