use iced::{
  container, executor, window, Application, Background, Color, Command, Container, Element, Length,
  Settings, Text,
};
use std::env;

pub struct ContainerStyle;

impl container::StyleSheet for ContainerStyle {
  fn style(&self) -> container::Style {
    container::Style {
      background: Some(Background::Color(Color::from_rgb8(73, 100, 122))),
      text_color: Some(Color::WHITE),
      ..container::Style::default()
    }
  }
}

pub fn main() -> iced::Result {
  Hello::run(Settings {
    antialiasing: true,
    window: window::Settings {
      size: (1280, 720),
      resizable: false,
      ..window::Settings::default()
    },
    ..Settings::default()
  })
}

struct Hello;

impl Application for Hello {
  type Executor = executor::Default;
  type Message = ();
  type Flags = ();

  fn new(_flags: ()) -> (Hello, Command<Self::Message>) {
    (Hello, Command::none())
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

    Container::new(text)
      .width(Length::Fill)
      .height(Length::Fill)
      .padding(20)
      .center_x()
      .center_y()
      .style(ContainerStyle)
      .into()
  }
}
