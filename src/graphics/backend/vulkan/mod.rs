mod geometry_manager;
mod renderer;
mod texture_manager;
use crate::graphics::backend::vulkan::geometry_manager::VkGeometryManager;
use crate::graphics::backend::vulkan::renderer::VkRenderer;
use crate::graphics::backend::vulkan::texture_manager::VkTextureManager;
use crate::graphics::manager::geometry_manager::GeometryManager;
use crate::graphics::manager::texture_manager::TextureManager;
use crate::graphics::{Graphics, PresentInfo, Render};

struct VkGraphics {
    texture_manager: VkTextureManager,
    geometry_manager: VkGeometryManager,
    renderer: VkRenderer,
}

impl Graphics for VkGraphics {
    fn get_mut_texture_manager(&mut self) -> &mut dyn TextureManager {
        &mut self.texture_manager
    }

    fn get_texture_manager(&self) -> &dyn TextureManager {
        &self.texture_manager
    }

    fn get_mut_geometry_manager(&mut self) -> &mut dyn GeometryManager {
        &mut self.geometry_manager
    }

    fn get_geometry_manager(&self) -> &dyn GeometryManager {
        &self.geometry_manager
    }

    fn get_mut_renderer(&mut self) -> &mut dyn Render {
        &mut self.renderer
    }

    fn get_renderer(&self) -> &dyn Render {
        &self.renderer
    }

    fn present(&mut self, info: PresentInfo) {
        unimplemented!()
    }
}
