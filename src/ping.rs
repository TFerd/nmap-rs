use std::net::{IpAddr, Ipv4Addr};

use iced::{
    Element,
    widget::{button, column, text},
};

pub struct Ping {
    pub ping_output: String,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Ping(IpAddr),
}

// #[derive(Default, Debug)]
// struct State {
//     pub ping_output: String,
// }

impl Ping {
    pub fn new() -> Self {
        Self {
            ping_output: "".to_string(),
        }
    }

    pub fn update(&mut self, message: Message) {
        println!("ping update called");
        match message {
            Message::Ping(ip_addr) => {
                self.ping_output = ip_addr.to_string();
                println!("todo: do this shit {:?}", ip_addr)
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        column![
            button("click me").on_press(Message::Ping(IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)))),
            text(self.ping_output.to_string())
        ]
        .into()
    }
}
