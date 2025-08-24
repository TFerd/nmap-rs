use iced::{Element, widget::button};

pub struct Home;

#[derive(Clone, Copy, Debug)]
pub enum Message {
    NavToPing,
}

impl Home {
    pub fn new() -> Self {
        Self
    }

    pub fn view(&self) -> Element<Message> {
        button("go ping").on_press(Message::NavToPing).into()
    }
    // this needs update for some reason?!
    // pub fn update(&self, action: Action) -> Action {
    //     action
    // }
}
