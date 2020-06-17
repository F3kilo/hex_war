use crate::screen_coords::ScreenCoords;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum State {
    Pressed,
    Released,
}

pub trait CursorRenderer {
    fn set_state(&mut self, state: State);
    fn render(&self, position: ScreenCoords);
}

#[derive(Debug)]
pub struct Cursor<Renderer> {
    pos: ScreenCoords,
    renderer: Renderer,
}

impl<Renderer: CursorRenderer> Cursor<Renderer> {
    pub fn new(pos: ScreenCoords, renderer: Renderer) -> Self {
        Self { pos, renderer }
    }

    pub fn get_pos(&self) -> ScreenCoords {
        self.pos
    }

    pub fn move_to(&mut self, pos: ScreenCoords) {
        self.pos = pos
    }

    pub fn set_state(&mut self, state: State) {
        self.renderer.set_state(state)
    }

    pub fn render(&self) {
        self.renderer.render(self.pos)
    }
}
