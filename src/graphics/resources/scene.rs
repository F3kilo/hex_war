use crate::graphics::resources::geometry::Geometry;
use crate::graphics::resources::texture::Texture;
use glam::{Mat4, Vec2};
use palette::Srgba;

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

#[derive(Debug, Copy, Clone, Default)]
pub struct Instance {
    pub transforms: Mat4,
    pub uv_transforms: UvTransforms,
    pub color: Srgba,
}

#[derive(Debug, Clone)]
pub struct TexturedGeometry {
    pub geometry: Geometry,
    pub texture: Texture,
    pub instance: Instance,
}

#[derive(Debug)]
pub struct Scene {
    textured_geometries: Vec<TexturedGeometry>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            textured_geometries: Vec::new(),
        }
    }

    pub fn add_textured_geometry(&mut self, inst: TexturedGeometry) {
        self.textured_geometries.push(inst)
    }

    pub fn clear(&mut self) {
        self.textured_geometries.clear()
    }

    pub fn get_textured_geometries(&self) -> impl Iterator<Item = &TexturedGeometry> {
        self.textured_geometries.iter()
    }
}
