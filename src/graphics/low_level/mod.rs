pub mod render_data;

use crate::graphics::manager::manage_geometries::ManageGeometries;
use crate::graphics::manager::manage_textures::{ManageTextures};
use glam::Mat4;
use crate::graphics::manager::manage_scenes::ManageScenes;

#[derive(Debug, Copy, Clone, Default)]
pub struct Transforms {
    pub instance: Mat4,
    pub uv: Mat4,
}

#[derive(Debug, Clone)]
pub struct PostEffect;

#[derive(Debug)]
pub struct RenderData;

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
}
