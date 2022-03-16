use iced::Font;

pub const ICONS: Font = Font::External {
    name: "Font Awesome Solid",
    bytes: include_bytes!("../font/fa-solid-900.ttf"),
};

pub const PLEXSANS: Font = Font::External {
    name: "IBM Plex Sans",
    bytes: include_bytes!("../font/IBMPlexSans-Regular.ttf"),
};

pub const PLEXSANSBOLD: Font = Font::External {
    name: "IBM Plex Sans Bold",
    bytes: include_bytes!("../font/IBMPlexSans-Bold.ttf"),
};

pub const PLEXMONO: Font = Font::External {
    name: "IBM Plex Mono",
    bytes: include_bytes!("../font/IBMPlexMono-Regular.ttf"),
};

pub enum Icon {
    Gear,
    Gauge,
    Stopwatch,
    CloudArrowUp,
    Microchip,
    MicrophoneLines,
    Microphone,
    VolumeOff,
    Shuffle,
}

impl ToString for Icon {
    fn to_string(&self) -> String {
        match self {
            Icon::Gear => '\u{f013}',
            Icon::Gauge => '\u{f624}',
            Icon::Stopwatch => '\u{f2f2}',
            Icon::CloudArrowUp => '\u{f0ee}',
            Icon::Microchip => '\u{f2db}',
            Icon::MicrophoneLines => '\u{f3c9}',
            Icon::Microphone => '\u{f130}',
            Icon::VolumeOff => '\u{f026}',
            Icon::Shuffle => '\u{f074}',
        }
        .to_string()
    }
}

impl From<Icon> for String {
    fn from(from: Icon) -> String {
        from.to_string()
    }
}
