use crate::graphics::manager::manage_geometries::GeometryId;
use crate::graphics::manager::manage_textures::TextureId;
use glam::{Mat4, Vec2};
use palette::Srgba;

#[derive(Debug, Clone)]
pub struct RenderData {
    textured_geometries: Vec<TexturedGeometryData>,
}

impl RenderData {
    pub fn new(textured_geometries: Vec<TexturedGeometryData>) -> Self {
        Self {
            textured_geometries,
        }
    }

    pub fn textured_geometries_iter(&self) -> impl Iterator<Item = &TexturedGeometryData> {
        self.textured_geometries.iter()
    }
}

#[derive(Debug, Clone)]
pub struct TexturedGeometryData {
    texture_id: TextureId,
    geometry_id: GeometryId,
    instance: Instance,
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Instance {
    pub transforms: Mat4,
    pub uv_transforms: UvTransforms,
    pub color: Srgba,
}

#[derive(Debug, Copy, Clone)]
pub struct UvTransforms {
    pub offset: Vec2,
    pub scale: Vec2,
}

impl Default for UvTransforms {
    fn default() -> Self {
        Self {
            offset: Vec2::zero(),
            scale: Vec2::one(),
        }
    }
}
