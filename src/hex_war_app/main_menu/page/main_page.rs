use crate::hex_war_app::main_menu::cursor::Cursor;
use slog::Logger;
use winit::event::{ElementState, MouseButton};

#[derive(Debug)]
pub enum MainPageEvent {
    StartGame,
    Settings,
    Exit,
}

#[derive(Debug)]
pub struct MainPage {
    logger: Logger,
}

impl MainPage {
    pub fn new(logger: Logger) -> Self {
        Self { logger }
    }

    pub fn cursor_moved(&mut self, cursor: &Cursor) {
        trace!(
            self.logger,
            "MainPage cursor moved to: {:?}",
            cursor.get_pos()
        );
    }

    pub fn button_used(
        &mut self,
        button: MouseButton,
        state: ElementState,
        cursor: &Cursor,
    ) -> Option<MainPageEvent> {
        trace!(
            self.logger,
            "MainPage button used: ({:?},{:?},{:?})",
            button,
            state,
            cursor.get_pos()
        );

        if button == MouseButton::Middle && state == ElementState::Pressed {
            return Some(MainPageEvent::Exit);
        }
        None
    }

    pub fn render(&self) {}
}
