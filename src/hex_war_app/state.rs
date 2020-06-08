use crate::hex_war_app::event::CursorEvent;
use crate::hex_war_app::main_menu::{MainMenu, MenuEvent};
use crate::hex_war_app::state::StateEvent::StartGame;

pub enum StateEvent {
    StartGame,
    Exit,
}

impl From<MenuEvent> for StateEvent {
    fn from(event: MenuEvent) -> Self {
        match event {
            MenuEvent::StartGame => StartGame,
            MenuEvent::Exit => StateEvent::Exit,
        }
    }
}

#[derive(Debug)]
pub enum State {
    Menu(MainMenu),
    Game,
}

impl State {
    pub fn new() -> Self {
        State::Menu(MainMenu::new())
    }

    pub fn cursor_used(&mut self, event: CursorEvent) -> Option<StateEvent> {
        match self {
            State::Menu(main_menu) => main_menu.cursor_used(event).map(|e| e.into()),
            State::Game => None,
        }
    }

    pub fn render(&self) {
        match self {
            State::Menu(main_menu) => main_menu.render(),
            State::Game => {}
        }
    }
}
