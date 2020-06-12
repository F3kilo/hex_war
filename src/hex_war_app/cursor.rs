use crate::coords::ScreenCoords;

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
}
