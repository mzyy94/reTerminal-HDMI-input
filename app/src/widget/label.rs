use crate::font::PLEXSANSBOLD;
use iced::{alignment, container, Background, Color, Container, Length, Text};

pub enum Label {
    Inactive,
    Active,
    Primary,
}

impl container::StyleSheet for Label {
    fn style(&self) -> container::Style {
        match self {
            Label::Active => container::Style {
                background: Some(Background::Color(Color::from_rgb8(200, 81, 89))),
                text_color: Some(Color::WHITE),
                ..container::Style::default()
            },
            Label::Inactive => container::Style {
                background: Some(Background::Color(Color::from_rgb8(71, 81, 60))),
                text_color: Some(Color::WHITE),
                ..container::Style::default()
            },
            Label::Primary => container::Style {
                background: Some(Background::Color(Color::from_rgb8(158, 242, 88))),
                text_color: Some(Color::WHITE),
                ..container::Style::default()
            },
        }
    }
}

pub fn text<'a, T>(text: &str, style: Label) -> Container<'a, T>
where
    T: Clone,
{
    let text = Text::new(text)
        .size(60)
        .font(PLEXSANSBOLD)
        .color(Color::WHITE)
        .vertical_alignment(alignment::Vertical::Center)
        .horizontal_alignment(alignment::Horizontal::Center);

    Container::new(text)
        .style(style)
        .padding(2)
        .align_x(alignment::Horizontal::Center)
        .align_y(alignment::Vertical::Center)
        .width(Length::Units(220))
        .height(Length::Units(60))
}

macro_rules! icon_text {
    ($icon:expr, $text:expr, $style:expr) => {
        Container::new(
            Row::new()
                .spacing(4)
                .align_items(Alignment::Center)
                .push(
                    Text::new($icon)
                        .size(24)
                        .font(crate::font::ICONS)
                        .color(Color::WHITE)
                        .vertical_alignment(alignment::Vertical::Center)
                        .horizontal_alignment(alignment::Horizontal::Center),
                )
                .push(
                    Text::new($text)
                        .size(36)
                        .font(crate::font::PLEXSANSBOLD)
                        .color(Color::WHITE)
                        .vertical_alignment(alignment::Vertical::Center)
                        .horizontal_alignment(alignment::Horizontal::Center),
                ),
        )
        .style($style)
        .padding(2)
        .align_x(alignment::Horizontal::Center)
        .align_y(alignment::Vertical::Center)
        .width(Length::Units(50))
        .height(Length::Units(80))
    };
    ($icon:expr, $style:expr) => {
        Container::new(
            Text::new($icon)
                .size(24)
                .font(crate::font::ICONS)
                .color(Color::WHITE)
                .vertical_alignment(alignment::Vertical::Center)
                .horizontal_alignment(alignment::Horizontal::Center),
        )
        .style($style)
        .padding(2)
        .align_x(alignment::Horizontal::Center)
        .align_y(alignment::Vertical::Center)
        .width(Length::Units(50))
        .height(Length::Units(80))
    };
}

pub(crate) use icon_text;
