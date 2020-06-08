use crate::hex_war_app::main_menu::cursor::Cursor;
use winit::event::{ElementState, MouseButton};

#[derive(Debug)]
pub enum MainPageEvent {
    StartGame,
    Settings,
    Exit,
}

#[derive(Debug)]
pub struct MainPage {}

impl MainPage {
    pub fn new() -> Self {
        Self {}
    }

    pub fn cursor_moved(&mut self, cursor: &Cursor) {}

    pub fn button_used(
        &mut self,
        button: MouseButton,
        state: ElementState,
        cursor: &Cursor,
    ) -> Option<MainPageEvent> {
        if button == MouseButton::Middle && state == ElementState::Pressed {
            return Some(MainPageEvent::Exit);
        }
        None
    }

    pub fn render(&self) {}
}
