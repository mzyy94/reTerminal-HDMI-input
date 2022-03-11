use iced::{executor, window, Application, Command, Element, Settings, Subscription};
use iced_native::Event;

use std::env;
use std::time::Instant;

mod font;
mod stream;
mod style;
mod view;
mod widget;

pub fn main() -> iced::Result {
    App::run(Settings {
        antialiasing: true,
        window: window::Settings {
            size: (1280, 720),
            resizable: false,
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

#[derive(Debug, Clone)]
pub enum View {
    Control,
    Setting,
}

struct App {
    control: view::control::App,
    setting: view::setting::App,
    view: View,
}

#[derive(Debug, Clone)]
pub enum Message {
    Event(Event),
    UpdateFrame(Instant),
    ChangeView(View),
    ToggleSecureInput(bool),
    InputChanged(String),
    StartStream(()),
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (App, Command<Self::Message>) {
        let control = view::control::App::new();
        let setting = view::setting::App::new();
        let view = View::Control;
        (
            App {
                control,
                setting,
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
            View::Control => self.control.subscription(),
            View::Setting => self.setting.subscription(),
        }
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::ChangeView(view) => {
                self.view = view;
                Command::none()
            }
            Message::StartStream(_) => {
                let url = self.setting.stream_url();
                self.control.start_stream(url).unwrap();
                Command::none()
            }
            _ => match self.view {
                View::Control => self.control.update(message),
                View::Setting => self.setting.update(message),
            },
        }
    }

    fn view(&mut self) -> Element<Message> {
        match self.view {
            View::Control => self.control.view(),
            View::Setting => self.setting.view(),
        }
    }
}
