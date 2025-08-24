use std::net::{IpAddr, Ipv4Addr};

use iced::{
    Element,
    widget::{button, column, text},
};

use crate::ping::Ping;

mod navbar;
mod ping;

struct State {
    page: Page,
}
enum Page {
    Ping(Ping),
    // Sniffer
}

fn main() -> iced::Result {
    iced::run("hello world", update, view)
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::Ping(ipAddr) => {
            println!("cawk: {:?}", ipAddr);
            // pagestate.ping_output = "ballsack".to_string()
            state.page(g).ma
        }
    }
}

fn view(page: Page) -> Element<Message> {
    match page {
        // Page::Ping(ping) => column![
        //     button(text("pres me"))
        //         .on_press(Message::Ping(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)))),
        //     text(ping.state.ping_output.to_string())
        // ]
        // .spacing(5)
        // .into(),
        Page::Ping(ping) => ping.view(),
        _ => text("where am i".to_string()),
    }
}

#[derive(Debug, Clone)]
enum Message {
    Ping(IpAddr),
}
