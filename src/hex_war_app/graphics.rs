#[derive(Debug)]
pub struct Sprite {}

impl Sprite {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(&self, renderer: &impl Renderer) {}
}

pub trait Renderer {}
