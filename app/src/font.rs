use iced::Font;

pub const ICONS: Font = Font::External {
  name: "MaterialIcons",
  bytes: include_bytes!("../font/MaterialIcons-Regular.ttf"),
};

pub const ROBOTO: Font = Font::External {
  name: "Roboto",
  bytes: include_bytes!("../font/Roboto-Black.ttf"),
};

pub const ROBOTOMONO: Font = Font::External {
  name: "RobotoMono",
  bytes: include_bytes!("../font/RobotoMono-Regular.ttf"),
};

pub enum Icon {
  MicOff,
  VideoCamOff,
  VolumeOff,
  Movie,
  PauseCircle,
  Settings,
  NetworkCheck,
  AvTimer,
  CloudUpload,
  DeveloperBoard,
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
      Icon::NetworkCheck => '\u{e640}',
      Icon::AvTimer => '\u{e01b}',
      Icon::CloudUpload => '\u{e2c3}',
      Icon::DeveloperBoard => '\u{e30d}',
    }
    .to_string()
  }
}

impl From<Icon> for String {
  fn from(from: Icon) -> String {
    from.to_string()
  }
}
