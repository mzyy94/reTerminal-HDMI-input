use iced::Font;

pub const ICONS: Font = Font::External {
  name: "MaterialIcons",
  bytes: include_bytes!("../res/MaterialIcons-Regular.ttf"),
};

pub const ROBOTO: Font = Font::External {
  name: "Roboto",
  bytes: include_bytes!("../res/Roboto-Black.ttf"),
};

pub enum Icon {
  MicOff,
  VideoCamOff,
  VolumeOff,
  Movie,
  PauseCircle,
  Settings,
}

impl ToString for Icon {
  fn to_string(&self) -> String {
    match self {
      Icon::MicOff => '\u{e02b}',
      Icon::VideoCamOff => '\u{e04c}',
      Icon::VolumeOff => '\u{e04f}',
      Icon::Movie => '\u{e02c}',
      Icon::PauseCircle => '\u{e1a2}',
      Icon::Settings => '\u{e8b8}',
    }
    .to_string()
  }
}

impl From<Icon> for String {
  fn from(from: Icon) -> String {
    from.to_string()
  }
}
