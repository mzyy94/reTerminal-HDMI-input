use iced::{
  button, executor, window, Alignment, Application, Color, Column, Command, Container, Element,
  Length, Row, Settings, Space, Text,
};
use std::env;

mod action;
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

impl Application for App {
  type Executor = executor::Default;
  type Message = ();
  type Flags = ();

  fn new(_flags: ()) -> (App, Command<Self::Message>) {
    (App::default(), Command::none())
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

  fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
    Command::none()
  }

  fn view(&mut self) -> Element<Self::Message> {
    let text = Text::new("Hello, world!");

    let video_area = Container::new(Space::new(Length::Units(1024), Length::Units(576)))
      .center_x()
      .style(style::PreviewArea);

    let side_actions: Element<_> = Column::new()
      .width(Length::Fill)
      .spacing(12)
      .align_items(Alignment::End)
      .push(Space::with_height(Length::Units(104)))
      .push(action::icon(
        &mut self.voice_off,
        "res/baseline_mic_off_white_48dp.png",
        action::IconButton::Active,
      ))
      .push(action::icon(
        &mut self.camera_off,
        "res/baseline_videocam_off_white_48dp.png",
        action::IconButton::Inactive,
      ))
      .push(action::icon(
        &mut self.sound_off,
        "res/baseline_volume_off_white_48dp.png",
        action::IconButton::Inactive,
      ))
      .push(action::icon(
        &mut self.video_off,
        "res/baseline_movie_white_48dp.png",
        action::IconButton::Inactive,
      ))
      .push(action::icon(
        &mut self.stream_off,
        "res/baseline_pause_circle_white_48dp.png",
        action::IconButton::Inactive,
      ))
      .into();

    let main_content: Element<_> = Row::new()
      .width(Length::Fill)
      .align_items(Alignment::Start)
      .push(video_area)
      .push(side_actions)
      .into();

    let bottom_actions: Element<_> = Row::new()
      .width(Length::Fill)
      .height(Length::Fill)
      .align_items(Alignment::End)
      .spacing(4)
      .push(action::icon(
        &mut self.settings,
        "res/baseline_settings_white_48dp.png",
        action::IconButton::Round,
      ))
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

    let content: Element<_> = Column::new()
      .height(Length::Fill)
      .spacing(2)
      .push(main_content)
      .push(text)
      .push(bottom_actions)
      .into();

    let content = if cfg!(debug_assertions) {
      content.explain(Color::BLACK)
    } else {
      content
    };

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
