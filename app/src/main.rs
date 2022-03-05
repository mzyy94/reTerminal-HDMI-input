use iced::{
  button, executor, image, window, Alignment, Application, Button, Color, Column, Command,
  Container, Element, Image, Length, Row, Settings, Space, Text,
};
use std::env;

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

struct App {
  voice_off: button::State,
  camera_off: button::State,
  sound_off: button::State,
  video_off: button::State,
  stream_off: button::State,
}

impl Application for App {
  type Executor = executor::Default;
  type Message = ();
  type Flags = ();

  fn new(_flags: ()) -> (App, Command<Self::Message>) {
    (
      App {
        voice_off: button::State::new(),
        camera_off: button::State::new(),
        sound_off: button::State::new(),
        video_off: button::State::new(),
        stream_off: button::State::new(),
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

  fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
    Command::none()
  }

  fn view(&mut self) -> Element<Self::Message> {
    let text = Text::new("Hello, world!");

    let video_area = Container::new(Space::new(Length::Units(1024), Length::Units(576)))
      .center_x()
      .style(style::PreviewArea);

    let action_button = |state, icon, active| {
      let image = Image::<image::Handle>::new(icon);
      let button = Button::new(state, image).style(if active {
        style::Button::Active
      } else {
        style::Button::Inactive
      });

      button
        .padding(8)
        .width(Length::Units(90))
        .height(Length::Units(80))
    };

    let side_actions = Column::new()
      .width(Length::Fill)
      .spacing(12)
      .align_items(Alignment::End)
      .push(Space::with_height(Length::Units(104)))
      .push(action_button(
        &mut self.voice_off,
        "res/baseline_mic_off_white_48dp.png",
        true,
      ))
      .push(action_button(
        &mut self.camera_off,
        "res/baseline_videocam_off_white_48dp.png",
        false,
      ))
      .push(action_button(
        &mut self.sound_off,
        "res/baseline_volume_off_white_48dp.png",
        false,
      ))
      .push(action_button(
        &mut self.video_off,
        "res/baseline_movie_white_48dp.png",
        false,
      ))
      .push(action_button(
        &mut self.stream_off,
        "res/baseline_pause_circle_white_48dp.png",
        false,
      ));

    let side_actions: Element<_> = side_actions.into();

    let main_content: Element<_> = Row::new()
      .width(Length::Fill)
      .align_items(Alignment::Start)
      .push(video_area)
      .push(side_actions)
      .into();

    let content: Element<_> = Column::new()
      .height(Length::Fill)
      .spacing(2)
      .push(main_content)
      .push(text)
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
