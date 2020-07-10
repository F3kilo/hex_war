use crate::graphics::manager::manage_geometries::ManageGeometries;
use crate::graphics::manager::manage_scenes::ManageScenes;
use crate::graphics::manager::manage_textures::ManageTextures;

pub mod vulkan;

#[derive(Clone)]
pub struct GraphicsSettings {}

impl GraphicsSettings {
    pub fn save(&self) {
        todo!()
    }

    pub fn load() -> Self {
        Self {}
    }
}

impl Default for GraphicsSettings {
    fn default() -> Self {
        Self {}
    }
}

#[derive(Debug)]
pub struct PresentInfo;

pub trait GraphicsBackend {
    fn get_mut_texture_manager(&mut self) -> &mut dyn ManageTextures;
    fn get_texture_manager(&self) -> &dyn ManageTextures;

    fn get_mut_geometry_manager(&mut self) -> &mut dyn ManageGeometries;
    fn get_geometry_manager(&self) -> &dyn ManageGeometries;

    fn get_mut_scene_manager(&mut self) -> &mut dyn ManageScenes;
    fn get_scene_manager(&self) -> &dyn ManageScenes;

    fn present(&mut self, info: PresentInfo);

    fn get_settings(&self) -> GraphicsSettings;
    fn update_settings(&mut self, settings: GraphicsSettings);
}
