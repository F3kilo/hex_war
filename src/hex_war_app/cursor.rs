use crate::graphics::resources::scene::Scene;
use crate::graphics::Camera;
use crate::math::screen_coords::ScreenCoords;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum State {
    Pressed,
    Released,
}

pub trait CursorRepresentation {
    fn set_state(&mut self, state: State);
    fn add_to_scene(&self, position: ScreenCoords, scene: &mut Scene, camera: &impl Camera);
}

#[derive(Debug)]
pub struct Cursor<Renderer> {
    pos: ScreenCoords,
    renderer: Renderer,
}

impl<Renderer: CursorRepresentation> Cursor<Renderer> {
    pub fn new(pos: ScreenCoords, renderer: Renderer) -> Self {
        Self { pos, renderer }
    }

    pub fn get_pos(&self) -> ScreenCoords {
        self.pos
    }

    pub fn move_to(&mut self, pos: ScreenCoords) {
        self.pos = pos
    }

    pub fn change_state(&mut self, state: State) {
        self.renderer.set_state(state)
    }

    pub fn add_to_scene(&self, scene: &mut Scene, camera: &impl Camera) {
        self.renderer.add_to_scene(self.pos, scene, camera)
    }
}
