mod geometry_manager;
mod texture_manager;
use crate::graphics::backend::vulkan::geometry_manager::VkGeometryManager;
use crate::graphics::backend::vulkan::texture_manager::VkTextureManager;
use crate::graphics::low_level::{GraphicsBackend, PresentInfo, RenderContext, RenderData};
use crate::graphics::manager::geometry_manager::ManageGeometry;
use crate::graphics::manager::texture_manager::{ManageTextures, TextureId};

pub struct VkGraphics {
    texture_manager: VkTextureManager,
    geometry_manager: VkGeometryManager,
}

impl VkGraphics {
    pub fn new() -> Self {
        let texture_manager = VkTextureManager::new();
        let geometry_manager = VkGeometryManager::new();
        Self {
            texture_manager,
            geometry_manager,
        }
    }
}

impl GraphicsBackend for VkGraphics {
    fn get_mut_texture_manager(&mut self) -> &mut dyn ManageTextures {
        &mut self.texture_manager
    }

    fn get_texture_manager(&self) -> &dyn ManageTextures {
        &self.texture_manager
    }

    fn get_mut_geometry_manager(&mut self) -> &mut dyn ManageGeometry {
        &mut self.geometry_manager
    }

    fn get_geometry_manager(&self) -> &dyn ManageGeometry {
        &self.geometry_manager
    }

    fn render(&mut self, context: RenderContext, data: RenderData) -> TextureId {
        unimplemented!()
    }

    fn present(&mut self, info: PresentInfo) {
        unimplemented!()
    }
}
