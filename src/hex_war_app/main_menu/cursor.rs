use glam::Vec2;

#[derive(Debug)]
pub struct Cursor {
    pos: Vec2,
}

impl Cursor {
    pub fn new(pos: Vec2) -> Self {
        Self { pos }
    }

    pub fn move_to(&mut self, new_pos: Vec2) {
        self.pos = new_pos
    }

    pub fn get_pos(&self) -> Vec2 {
        self.pos
    }

    pub fn render(&self) {}
}
