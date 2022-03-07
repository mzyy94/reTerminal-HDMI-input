use crate::font::{Icon, ICONS, ROBOTO};
use iced::{alignment, button, Background, Button, Color, Length, Text};

pub enum IconButton {
  Active,
  Inactive,
  Round,
}

impl button::StyleSheet for IconButton {
  fn active(&self) -> button::Style {
    match self {
      IconButton::Active => button::Style {
        background: Some(Background::Color(Color::from_rgb8(94, 182, 238))),
        text_color: Color::WHITE,
        ..button::Style::default()
      },
      IconButton::Inactive => button::Style {
        background: Some(Background::Color(Color::from_rgb8(180, 180, 180))),
        text_color: Color::WHITE,
        ..button::Style::default()
      },

      IconButton::Round => button::Style {
        background: Some(Background::Color(Color::from_rgb8(119, 139, 143))),
        text_color: Color::WHITE,
        border_radius: 80.0,
        ..button::Style::default()
      },
    }
  }

  fn disabled(&self) -> button::Style {
    self.active()
  }
}

pub fn icon<'a, T>(state: &'a mut button::State, icon: Icon, style: IconButton) -> Button<'a, T>
where
  T: Clone,
{
  let text = Text::new(icon)
    .font(ICONS)
    .width(Length::Units(64))
    .horizontal_alignment(alignment::Horizontal::Center)
    .vertical_alignment(alignment::Vertical::Center)
    .size(64);
  let width = match style {
    IconButton::Round => 100,
    _ => 90,
  };
  let height = match style {
    IconButton::Round => 100,
    _ => 80,
  };
  let button = Button::new(state, text).style(style);

  button
    .padding(8)
    .width(Length::Units(width))
    .height(Length::Units(height))
}

pub enum LabelButton {
  Action,
  Primary,
}

impl button::StyleSheet for LabelButton {
  fn active(&self) -> button::Style {
    match self {
      LabelButton::Action => button::Style {
        background: Some(Background::Color(Color::from_rgb8(71, 81, 60))),
        text_color: Color::WHITE,
        ..button::Style::default()
      },
      LabelButton::Primary => button::Style {
        background: Some(Background::Color(Color::from_rgb8(158, 242, 88))),
        text_color: Color::WHITE,
        ..button::Style::default()
      },
    }
  }

  fn disabled(&self) -> button::Style {
    self.active()
  }
}

pub fn text<'a, T>(state: &'a mut button::State, text: &str, style: LabelButton) -> Button<'a, T>
where
  T: Clone,
{
  let text = Text::new(text)
    .size(60)
    .font(ROBOTO)
    .color(Color::WHITE)
    .vertical_alignment(alignment::Vertical::Center)
    .horizontal_alignment(alignment::Horizontal::Center);
  let button = Button::new(state, text).style(style);

  button
    .padding(2)
    .width(Length::Units(220))
    .height(Length::Units(60))
}
