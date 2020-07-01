use crate::graphics::manager::geometry_manager::GeometryManager;
use crate::graphics::manager::texture_manager::{TextureId, TextureManager};
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

pub trait Render {
    fn render(&mut self, context: RenderContext, data: RenderData) -> TextureId;
}

pub trait Present {
    fn present(&mut self, info: PresentInfo);
}

pub trait ProvideTextureManager {
    fn get_mut_texture_manager(&mut self) -> &mut dyn TextureManager;
    fn get_texture_manager(&self) -> &dyn TextureManager;
}

pub trait ProvideGeometryManager {
    fn get_mut_geometry_manager(&mut self) -> &mut dyn GeometryManager;
    fn get_geometry_manager(&self) -> &dyn GeometryManager;
}

pub trait ProvideRenderer {
    fn get_mut_renderer(&mut self) -> &mut dyn Render;
    fn get_renderer(&self) -> &dyn Render;
}

pub trait Graphics:
    ProvideTextureManager + ProvideGeometryManager + ProvideRenderer + Present
{
}
