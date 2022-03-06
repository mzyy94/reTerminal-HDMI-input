use iced::{alignment, button, Background, Button, Color, Font, Length, Text};

pub enum IconButton {
  Active,
  Inactive,
  Round,
}

const ICONS: Font = Font::External {
  name: "MaterialIcons",
  bytes: include_bytes!("../res/MaterialIcons-Regular.ttf"),
};

pub enum Icon {
  MicOff,
  VideoCamOff,
  VolumeOff,
  Movie,
  PauseCircle,
  Settings,
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

      IconButton::Round => button::Style {
        background: Some(Background::Color(Color::from_rgb8(200, 200, 200))),
        text_color: Color::WHITE,
        border_radius: 80.0,
        ..button::Style::default()
      },
    }
  }
}

pub fn icon<'a>(state: &'a mut button::State, icon: Icon, style: IconButton) -> Button<'a, ()> {
  let unicode = match icon {
    Icon::MicOff => '\u{e02b}',
    Icon::VideoCamOff => '\u{e04c}',
    Icon::VolumeOff => '\u{e04f}',
    Icon::Movie => '\u{e02c}',
    Icon::PauseCircle => '\u{e1a2}',
    Icon::Settings => '\u{e8b8}',
  };
  let text = Text::new(unicode.to_string())
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

pub fn text<'a>(state: &'a mut button::State, text: &str, style: LabelButton) -> Button<'a, ()> {
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
