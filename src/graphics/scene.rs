use crate::graphics::geometry::Geometry;
use crate::graphics::low_level::{
    ProvideGeometryManager, ProvideRenderer, ProvideTextureManager, Render, RenderContext,
    RenderData,
};
use crate::graphics::texture::{Texture, UniqueTexture};
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
pub struct TexturedGeometry<G, T>
where
    G: ProvideGeometryManager,
    T: ProvideTextureManager,
{
    pub geometry: Geometry<G>,
    pub texture: Texture<T>,
    pub instance: Instance,
}

pub struct Scene<G, T>
where
    G: ProvideGeometryManager,
    T: ProvideTextureManager,
{
    textured_geometries: Vec<TexturedGeometry<G, T>>,
}

impl<G, T> Scene<G, T>
where
    G: ProvideGeometryManager,
    T: ProvideTextureManager,
{
    pub fn new() -> Self {
        Self {
            textured_geometries: Vec::new(),
        }
    }

    pub fn add_textured_geometry(&mut self, textured_geometry: TexturedGeometry<G, T>) {
        self.textured_geometries.push(textured_geometry)
    }

    pub fn render(
        &mut self,
        renderer: &mut impl Render,
        texture_manager_provider: T,
    ) -> UniqueTexture<T> {
        let render_context = self.get_render_context();
        let render_data = self.get_render_data();
        let texture_id = renderer.render(render_context, render_data);
        UniqueTexture::from_raw(texture_id, texture_manager_provider)
    }

    pub fn clear(&mut self) {
        self.textured_geometries.clear()
    }

    fn get_render_context(&self) -> RenderContext {
        unimplemented!()
    }

    fn get_render_data(&self) -> RenderData {
        unimplemented!()
    }
}
