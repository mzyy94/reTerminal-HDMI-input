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

#[derive(Default)]
struct App {
    control: view::control::App,
}

#[derive(Debug, Clone)]
pub enum Message {
    Event(Event),
    UpdateFrame(Instant),
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (App, Command<Self::Message>) {
        let control = view::control::App::new();
        (App { control }, Command::none())
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
        self.control.subscription()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        self.control.update(message)
    }

    fn view(&mut self) -> Element<Message> {
        self.control.view()
    }
}
