use iced::{Element, widget::button};

pub struct Home;

#[derive(Clone, Copy, Debug)]
pub enum Action {
    GoPing,
}

impl Home {
    pub fn new() -> Self {
        Self
    }

    pub fn view(&self) -> Element<Action> {
        button("go ping").on_press(Action::GoPing).into()
    }
    // this needs update for some reason?!
}
