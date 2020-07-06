mod geometry_manager;
mod texture_manager;
mod utils;
mod vk;
use crate::graphics::backend::vulkan::geometry_manager::VkGeometryManager;
use crate::graphics::backend::vulkan::texture_manager::VkTextureManager;
use crate::graphics::low_level::{GraphicsBackend, PresentInfo, RenderContext, RenderData};
use crate::graphics::manager::manage_geometries::ManageGeometries;
use crate::graphics::manager::manage_textures::{ManageTextures, TextureId};

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

    fn get_mut_geometry_manager(&mut self) -> &mut dyn ManageGeometries {
        &mut self.geometry_manager
    }

    fn get_geometry_manager(&self) -> &dyn ManageGeometries {
        &self.geometry_manager
    }

    fn render(&mut self, context: RenderContext, data: RenderData) -> TextureId {
        unimplemented!()
    }

    fn present(&mut self, info: PresentInfo) {
        unimplemented!()
    }
}
