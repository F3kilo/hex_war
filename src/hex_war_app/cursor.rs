use crate::hex_war_app::event::CursorEvent;
use crate::hex_war_app::graphics::{Renderer, Sprite};
use glam::Vec2;

#[derive(Debug)]
pub struct Cursor {
    pos: Vec2,
    sprite: Sprite,
}

impl Cursor {
    pub fn new(pos: Vec2, sprite: Sprite) -> Self {
        Self { pos, sprite }
    }

    pub fn get_pos(&self) -> Vec2 {
        self.pos
    }

    pub fn render(&self, renderer: &impl Renderer) {
        self.sprite.render(renderer)
    }

    pub fn process_event(&mut self, event: CursorEvent) {
        match event {
            CursorEvent::MoveTo(new_pos) => self.pos = new_pos,
            CursorEvent::ButtonUse(_, _) => {}
        }
    }
}
