struct Navbar;

enum Action {
    NavToPing,
    NavToHome,
}

impl Navbar {
    fn update(&self, action: Action) -> Action {
        match action {
            Action::NavToHome => Action::NavToHome,
            Action::NavToPing => Action::NavToPing,
        }
    }

    // fn view () {}  // show this jawn
}
