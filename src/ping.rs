use std::net::{IpAddr, Ipv4Addr};

use iced::{
    Element,
    widget::{button, column, text},
};

pub struct Ping {
    pub ping_output: String,
}

#[derive(Debug, Clone, Copy)]
pub enum PingMessage {
    Ping(IpAddr),
}

// #[derive(Default, Debug)]
// struct State {
//     pub ping_output: String,
// }

impl Ping {
    pub fn view(&self) -> Element<PingMessage> {
        column![
            button("cokc").on_press(PingMessage::Ping(IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)))),
            text(self.ping_output.to_string())
        ]
        .into()
    }

    pub fn update(&mut self, message: PingMessage) {
        match message {
            PingMessage::Ping(ip_addr) => {
                println!("todo: do this shit {:?}", ip_addr)
            }
        }
    }
}
