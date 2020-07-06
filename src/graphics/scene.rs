use crate::graphics::camera::Camera;
use crate::graphics::geometry::Geometry;
use crate::graphics::low_level::{RenderContext, RenderData, SceneTransforms};
use crate::graphics::texture::{Texture, UniqueTexture};
use crate::graphics::Graphics;
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

#[derive(Clone, Debug)]
pub struct Scene {
    textured_geometries: Vec<TexturedGeometry>,
    transforms: Mat4,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            textured_geometries: Vec::new(),
            transforms: Mat4::default(),
        }
    }

    pub fn add_textured_geometry(&mut self, textured_geometry: TexturedGeometry) {
        self.textured_geometries.push(textured_geometry)
    }

    pub fn clear(&mut self) {
        self.textured_geometries.clear()
    }

    pub fn render(&self, camera: &impl Camera, graphics: &mut Graphics) -> Texture {
        let render_context = RenderContext {
            scene_transforms: SceneTransforms {
                world: Default::default(),
                view: camera.get_view_transform(),
                proj: camera.get_proj_transform(),
            },
        };
        let texture_id = graphics.render(render_context);
        Texture::new(
            UniqueTexture::from_raw(texture_id, graphics.get_texture_manager().clone())
                .expect("Texture manager doesn't contains just presented texture."),
        )
    }
}
