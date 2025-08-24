use iced::Element;

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
    // anything in parenthesis are just dragging up the Messages from the pages. why? look below lil bro
    Home(home::Message),
    Ping(ping::Message), // pings messages dragged up here
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
    println!("Main method received message {:?}", message);
    match message {
        Message::Home(_) => state.screen = Screen::Ping(Ping::new()), // this says if ANY home message, go to ping screen
        Message::Ping(message) => {
            // if we're on the ping screen, update ping, otherwise do nothing
            if let Screen::Ping(ping) = &mut state.screen {
                ping.update(message);
                // if we need to do anything on the home screen, capture the result of
                // ping.update(message) and run a switch statement on the result
            }
        }
    }
}

fn view(state: &State) -> Element<Message> {
    match &state.screen {
        Screen::Home(home) => home.view().map(Message::Home),
        Screen::Ping(ping) => ping.view().map(Message::Ping),
    }
}
