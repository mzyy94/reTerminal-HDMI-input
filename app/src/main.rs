use iced::{
  executor, window, Alignment, Application, Color, Column, Command, Container, Element, Length,
  Row, Settings, Space, Text,
};
use std::env;

mod style;

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

    let video_area = Container::new(Space::new(Length::Units(1024), Length::Units(576)))
      .center_x()
      .style(style::PreviewArea);

    let main_content: Element<_> = Row::new()
      .width(Length::Fill)
      .spacing(20)
      .align_items(Alignment::Start)
      .push(video_area)
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
