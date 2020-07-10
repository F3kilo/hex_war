mod geometry_manager;
mod scene_manager;
mod texture_manager;
mod utils;
mod vk;
use crate::graphics::backend::vulkan::geometry_manager::VkGeometryManager;
use crate::graphics::backend::vulkan::scene_manager::VkSceneManager;
use crate::graphics::backend::vulkan::texture_manager::VkTextureManager;
use crate::graphics::backend::{GraphicsBackend, PresentInfo};
use crate::graphics::manager::manage_geometries::ManageGeometries;
use crate::graphics::manager::manage_scenes::ManageScenes;
use crate::graphics::manager::manage_textures::ManageTextures;

pub struct VkGraphics {
    texture_manager: VkTextureManager,
    geometry_manager: VkGeometryManager,
    scene_manager: VkSceneManager,
}

impl VkGraphics {
    pub fn new() -> Self {
        let texture_manager = VkTextureManager::new();
        let geometry_manager = VkGeometryManager::new();
        let scene_manager = VkSceneManager::new();
        Self {
            texture_manager,
            geometry_manager,
            scene_manager,
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

    fn get_mut_scene_manager(&mut self) -> &mut dyn ManageScenes {
        &mut self.scene_manager
    }

    fn get_scene_manager(&self) -> &dyn ManageScenes {
        &self.scene_manager
    }

    fn present(&mut self, info: PresentInfo) {
        todo!()
    }
}
