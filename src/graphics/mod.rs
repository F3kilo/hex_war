use crate::graphics::resources::scene::Scene;
use crate::graphics::resources::texture::Texture;
use crate::graphics::resources::{geometry, texture};
use glam::Mat4;
use std::error::Error;
use std::fmt;
use std::path::PathBuf;

pub mod primitive;
pub mod resources;

#[derive(Debug)]
pub enum LoadError {}

impl Error for LoadError {}
impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Can't load resource")
    }
}

#[derive(Debug)]
pub enum NotFoundError {}

impl Error for NotFoundError {}
impl fmt::Display for NotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Resource with specified ID is not found.")
    }
}

pub trait RenderBackend {
    fn create_texture(&self, path: PathBuf) -> texture::Texture;
    fn create_geometry(&self, path: PathBuf) -> geometry::Geometry;
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Transforms {
    pub instance: Mat4,
    pub uv: Mat4,
}

pub struct Renderer {
    backend: Box<dyn RenderBackend>,
}

#[derive(Debug, Clone)]
pub struct PostEffect;

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

impl Renderer {
    fn new(backend: Box<dyn RenderBackend>) -> Self {
        Self { backend }
    }

    fn render(
        &mut self,
        context: &RenderContext,
        scene: &Scene,
        post_effects: &impl Iterator<Item = PostEffect>,
    ) { // TODO: return Texture
    }
    fn present(
        &mut self,
        images: &impl Iterator<Item = Texture>,
        post_effects: &impl Iterator<Item = PostEffect>,
    ) {
    }
}
