use crate::graphics::Camera;
use crate::math::screen_coords::ScreenCoords;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum State {
    Pressed,
    Released,
}

pub trait CursorRepresentation {
    fn set_state(&mut self, state: State);
    fn add_to_scene(&self, position: ScreenCoords, screen_size: ScreenCoords, camera: &impl Camera);
}

#[derive(Debug)]
pub struct Cursor {
    pos: ScreenCoords,
}

impl Cursor {
    pub fn new(pos: ScreenCoords) -> Self {
        Self { pos }
    }

    pub fn get_pos(&self) -> ScreenCoords {
        self.pos
    }

    pub fn move_to(&mut self, pos: ScreenCoords) {
        self.pos = pos
    }

    pub fn change_state(&mut self, state: State) {}
}
