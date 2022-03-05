use iced::{button, image, Background, Button, Color, Image, Length};

pub enum ButtonState {
  Active,
  Inactive,
}

impl button::StyleSheet for ButtonState {
  fn active(&self) -> button::Style {
    match self {
      ButtonState::Active => button::Style {
        background: Some(Background::Color(Color::from_rgb8(114, 202, 255))),
        border_radius: 10.0,
        text_color: Color::WHITE,
        ..button::Style::default()
      },
      ButtonState::Inactive => button::Style {
        background: Some(Background::Color(Color::from_rgb8(200, 200, 200))),
        border_radius: 10.0,
        text_color: Color::WHITE,
        ..button::Style::default()
      },
    }
  }
}

pub fn icon<'a>(state: &'a mut button::State, icon: &str, active: bool) -> Button<'a, ()> {
  let image = Image::<image::Handle>::new(icon);
  let button = Button::new(state, image).style(if active {
    ButtonState::Active
  } else {
    ButtonState::Inactive
  });

  button
    .padding(8)
    .width(Length::Units(90))
    .height(Length::Units(80))
}
