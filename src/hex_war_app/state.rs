use crate::hex_war_app::main_menu::MainMenu;
use slog::Logger;

#[derive(Debug)]
pub enum State {
    Menu(MainMenu),
    Game,
    Finished,
}

impl State {
    pub fn new(logger: Logger) -> Self {
        State::Menu(MainMenu::new(logger))
    }
}
