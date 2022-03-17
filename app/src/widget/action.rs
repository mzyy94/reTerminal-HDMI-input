use crate::font::{Icon, ICONS};
use iced::{alignment, button, Background, Button, Color, Length, Text};

pub enum IconButton {
    Round,
}

impl button::StyleSheet for IconButton {
    fn active(&self) -> button::Style {
        match self {
            IconButton::Round => button::Style {
                background: Some(Background::Color(Color::from_rgb8(119, 139, 143))),
                text_color: Color::WHITE,
                border_radius: 80.0,
                ..button::Style::default()
            },
        }
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
        .size(48);
    let width = 100;
    let height = 100;
    let button = Button::new(state, text).style(style);

    button
        .padding(8)
        .width(Length::Units(width))
        .height(Length::Units(height))
}
