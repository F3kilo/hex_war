use crate::hex_war_app::main_menu::MainMenu;
use crate::hex_war_app::state::StateEvent::StartGame;
use slog::Logger;

#[derive(Debug)]
pub enum StateEvent {
    StartGame,
    Exit,
}

#[derive(Debug)]
pub enum State {
    Menu(MainMenu),
    Game,
}

impl State {
    pub fn new(logger: Logger) -> Self {
        State::Menu(MainMenu::new(logger))
    }
}
