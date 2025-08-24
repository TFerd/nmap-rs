use iced::{Element, widget::button};

use crate::{home::Home, ping::Ping};

mod home;
mod navbar;
mod ping;

struct State {
    screen: Screen,
}

impl Default for State {
    fn default() -> Self {
        Self {
            screen: Screen::Home(Home::new()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Home(home::Action),
    Ping(ping::Message),
}

enum Screen {
    Home(Home),
    Ping(Ping),
    // Sniffer,
}

fn main() -> iced::Result {
    iced::application("hello world", update, view).run()
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::Home(message) => {
            if let Screen::Home(home) = &mut state.screen {
                // let action = home
            }
        }
        Message::Ping(_) => state.screen = Screen::Ping(Ping::new()),
    }
}

fn view(state: &State) -> Element<Message> {
    match &state.screen {
        Screen::Home(home) => home.view().map(Message::Home),
        Screen::Ping(ping) => ping.view().map(Message::Ping),
    }
}
