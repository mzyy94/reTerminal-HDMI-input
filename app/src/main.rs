use iced::{executor, window, Application, Command, Element, Settings, Subscription};

use std::env;

mod font;
mod ingest;
mod setting;
mod stream;
mod style;
mod view;
mod widget;

use crate::view::{View, ViewApp};

use lazy_static::lazy_static;
use std::sync::RwLock;

lazy_static! {
    static ref SETTINGS: RwLock<setting::Settings> = RwLock::new(setting::Settings::new());
}

pub fn main() -> iced::Result {
    let font = if let iced::Font::External { bytes, .. } = font::PLEXSANS {
        Some(bytes)
    } else {
        None
    };

    App::run(Settings {
        antialiasing: true,
        window: window::Settings {
            size: (1280, 720),
            resizable: false,
            ..window::Settings::default()
        },
        default_font: font,
        ..Settings::default()
    })
}

struct App {
    control: view::control::App,
    setting: view::setting::App,
    ingests: view::ingests::App,
    view: View,
}

#[derive(Debug, Clone)]
pub enum Message {
    ChangeView(View),
    StartStream(()),
    ViewMessage(view::ViewMessage),
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (App, Command<Self::Message>) {
        let control = view::control::App::new();
        let setting = view::setting::App::new();
        let ingests = view::ingests::App::new();
        let view = View::Control;

        (
            App {
                control,
                setting,
                ingests,
                view,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("broadcast-terminal")
    }

    fn mode(&self) -> window::Mode {
        let args: Vec<String> = env::args().collect();
        if args.len() > 1 && &args[1] == "--fullscreen" {
            window::Mode::Fullscreen
        } else {
            window::Mode::Windowed
        }
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        match self.view {
            View::Control => self
                .control
                .subscription()
                .map(view::ViewMessage::Control)
                .map(crate::Message::ViewMessage),
            View::Setting => self
                .setting
                .subscription()
                .map(view::ViewMessage::Setting)
                .map(crate::Message::ViewMessage),
            View::Ingests => self
                .ingests
                .subscription()
                .map(view::ViewMessage::Ingests)
                .map(crate::Message::ViewMessage),
        }
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::ChangeView(view) => {
                let prev = self.view.clone();
                self.view = view;
                match self.view {
                    View::Ingests => {
                        return Command::perform(ingest::Twitch::get_ingests(), |twitch| {
                            Message::ViewMessage(view::ViewMessage::Ingests(
                                view::ingests::Message::FetchIngest(twitch),
                            ))
                        });
                    }
                    View::Setting => match prev {
                        View::Ingests => {
                            self.setting.refresh();
                        }
                        _ => {}
                    },
                    View::Control => {
                        self.control.reload_setting();
                    }
                }
                Command::none()
            }
            Message::StartStream(_) => {
                self.control.start_stream().unwrap();
                Command::none()
            }
            Message::ViewMessage(message) => match message {
                view::ViewMessage::Control(message) => self.control.update(message),
                view::ViewMessage::Setting(message) => self.setting.update(message),
                view::ViewMessage::Ingests(message) => self.ingests.update(message),
            },
        }
    }

    fn view(&mut self) -> Element<Message> {
        match self.view {
            View::Control => self.control.view(),
            View::Setting => self.setting.view(),
            View::Ingests => self.ingests.view(),
        }
    }
}
