use iced::{container, Background, Color};

pub struct Container;

impl container::StyleSheet for Container {
  fn style(&self) -> container::Style {
    container::Style {
      background: Some(Background::Color(Color::from_rgb8(73, 100, 122))),
      text_color: Some(Color::WHITE),
      ..container::Style::default()
    }
  }
}

pub struct PreviewArea;

impl container::StyleSheet for PreviewArea {
  fn style(&self) -> container::Style {
    container::Style {
      background: Some(Background::Color(Color::from_rgb8(78, 78, 78))),
      text_color: Some(Color::WHITE),
      border_radius: 10.0,
      ..container::Style::default()
    }
  }
}
