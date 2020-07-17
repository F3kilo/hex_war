mod async_tasker;
mod geometry_manager;
mod scene_manager;
mod texture;
mod texture_manager;
mod utils;
mod vk;
use crate::graphics::backend::vulkan::geometry_manager::VkGeometryManager;
use crate::graphics::backend::vulkan::scene_manager::VkSceneManager;
use crate::graphics::backend::vulkan::texture::loader::VkTextureLoader;
use crate::graphics::backend::vulkan::texture_manager::VkTextureManager;
use crate::graphics::backend::{GraphicsBackend, GraphicsSettings, PresentInfo};
use crate::graphics::manager::manage_geometries::ManageGeometries;
use crate::graphics::manager::manage_scenes::ManageScenes;
use crate::graphics::manager::manage_textures::ManageTextures;
use slog::Logger;

pub struct VkGraphics {
    logger: Logger,
    settings: GraphicsSettings,
    texture_manager: VkTextureManager,
    geometry_manager: VkGeometryManager,
    scene_manager: VkSceneManager,
}

impl VkGraphics {
    pub fn new(logger: Logger, settings: GraphicsSettings) -> Self {
        let texture_loader = VkTextureLoader::new();
        let texture_manager = VkTextureManager::new(logger.clone(), texture_loader);
        let geometry_manager = VkGeometryManager::new(logger.clone());
        let scene_manager = VkSceneManager::new(logger.clone());
        Self {
            logger,
            texture_manager,
            geometry_manager,
            scene_manager,
            settings,
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

    fn get_settings(&self) -> GraphicsSettings {
        self.settings.clone()
    }

    fn update_settings(&mut self, settings: GraphicsSettings) {
        self.settings = settings
    }
}
