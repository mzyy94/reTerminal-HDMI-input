use crate::font::{Icon, ICONS, PLEXSANSBOLD};
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

pub enum LabelButton {
    Action,
    Inactive,
    Active,
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
            LabelButton::Active => button::Style {
                background: Some(Background::Color(Color::from_rgb8(200, 81, 89))),
                text_color: Color::WHITE,
                ..button::Style::default()
            },
            LabelButton::Inactive => button::Style {
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
        .font(PLEXSANSBOLD)
        .color(Color::WHITE)
        .vertical_alignment(alignment::Vertical::Center)
        .horizontal_alignment(alignment::Horizontal::Center);
    let button = Button::new(state, text).style(style);

    button
        .padding(2)
        .width(Length::Units(220))
        .height(Length::Units(60))
}
