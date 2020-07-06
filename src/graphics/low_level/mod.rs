pub mod render_data;

use crate::graphics::manager::manage_geometries::ManageGeometries;
use crate::graphics::manager::manage_textures::{ManageTextures, TextureId};
use glam::Mat4;

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

#[derive(Debug, Copy, Clone, Default)]
pub struct SceneTransforms {
    pub world: Mat4,
    pub view: Mat4,
    pub proj: Mat4,
}

#[derive(Debug, Copy, Clone, Default)]
pub struct RenderContext {
    pub scene_transforms: SceneTransforms,
}

pub trait GraphicsBackend {
    fn get_mut_texture_manager(&mut self) -> &mut dyn ManageTextures;
    fn get_texture_manager(&self) -> &dyn ManageTextures;

    fn get_mut_geometry_manager(&mut self) -> &mut dyn ManageGeometries;
    fn get_geometry_manager(&self) -> &dyn ManageGeometries;

    fn render(&mut self, context: RenderContext, data: RenderData) -> TextureId;

    fn present(&mut self, info: PresentInfo);
}
