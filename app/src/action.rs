use iced::{alignment, button, image, Background, Button, Color, Font, Image, Length, Text};

pub enum IconButton {
  Active,
  Inactive,
}

impl button::StyleSheet for IconButton {
  fn active(&self) -> button::Style {
    match self {
      IconButton::Active => button::Style {
        background: Some(Background::Color(Color::from_rgb8(114, 202, 255))),
        text_color: Color::WHITE,
        ..button::Style::default()
      },
      IconButton::Inactive => button::Style {
        background: Some(Background::Color(Color::from_rgb8(200, 200, 200))),
        text_color: Color::WHITE,
        ..button::Style::default()
      },
    }
  }
}

pub fn icon<'a>(state: &'a mut button::State, icon: &str, active: bool) -> Button<'a, ()> {
  let image = Image::<image::Handle>::new(icon);
  let button = Button::new(state, image).style(if active {
    IconButton::Active
  } else {
    IconButton::Inactive
  });

  button
    .padding(8)
    .width(Length::Units(90))
    .height(Length::Units(80))
}

const ROBOTO: Font = Font::External {
  name: "Roboto",
  bytes: include_bytes!("../res/Roboto-Black.ttf"),
};

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
}

pub fn text<'a>(state: &'a mut button::State, text: &str, action: bool) -> Button<'a, ()> {
  let text = Text::new(text)
    .size(60)
    .font(ROBOTO)
    .color(Color::WHITE)
    .vertical_alignment(alignment::Vertical::Center)
    .horizontal_alignment(alignment::Horizontal::Center);
  let button = Button::new(state, text).style(if action {
    LabelButton::Action
  } else {
    LabelButton::Primary
  });

  button
    .padding(2)
    .width(Length::Units(220))
    .height(Length::Units(60))
}
