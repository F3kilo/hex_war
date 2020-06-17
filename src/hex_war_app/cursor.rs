use crate::coords::ScreenCoords;

pub trait CursorRenderer {
    fn render(&self, pos: ScreenCoords);
}

#[derive(Debug)]
pub struct Cursor<R> {
    pos: ScreenCoords,
    renderer: R,
}

impl<R: CursorRenderer> Cursor<R> {
    pub fn new(pos: ScreenCoords, renderer: R) -> Self {
        Self { pos, renderer }
    }

    pub fn get_pos(&self) -> ScreenCoords {
        self.pos
    }

    pub fn move_to(&mut self, pos: ScreenCoords) {
        self.pos = pos
    }

    pub fn draw(&self) {
        self.renderer.render(self.get_pos())
    }
}
