use crate::graphics::low_level::{Render, RenderContext, RenderData};
use crate::graphics::manager::texture_manager::TextureId;

pub struct VkRenderer {}

impl VkRenderer {
    pub fn new() -> Self {
        Self {}
    }
}

impl Render for VkRenderer {
    fn render(&mut self, context: RenderContext, data: RenderData) -> TextureId {
        unimplemented!()
    }
}
