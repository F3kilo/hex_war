use crate::graphics::backend::RenderBackend;
use crate::graphics::resources::scene::Scene;
use crate::graphics::resources::texture::Texture;
use crate::graphics::resources::{geometry, scene, texture};
use crate::math::screen_coords::ScreenCoords;
use crate::math::world_coords::WorldCoords;
use glam::Mat4;
use std::error::Error;
use std::fmt;
use std::path::PathBuf;

pub mod backend;
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
    pub fn new(backend: Box<dyn RenderBackend>) -> Self {
        Self { backend }
    }

    pub fn create_texture(&self, path: PathBuf) -> Result<texture::Texture, LoadError> {
        self.backend.create_texture(path)
    }
    pub fn create_geometry(&self, path: PathBuf) -> Result<geometry::Geometry, LoadError> {
        self.backend.create_geometry(path)
    }
    pub fn create_scene(&self) -> scene::Scene {
        self.backend.create_scene()
    }

    pub fn render(
        &mut self,
        context: &RenderContext,
        scene: &Scene,
        post_effects: &impl Iterator<Item = PostEffect>,
    ) { // TODO: return Texture
    }

    pub fn present(
        &mut self,
        images: &impl Iterator<Item = Texture>,
        post_effects: &impl Iterator<Item = PostEffect>,
    ) {
    }
}

pub type Depth = f32;

pub trait Camera {
    fn to_world(&self, point: ScreenCoords, depth: Depth, screen_size: ScreenCoords)
        -> WorldCoords;
    fn to_screen(&self, world: WorldCoords, screen_size: ScreenCoords) -> (ScreenCoords, Depth);

    fn get_view_transform(&self) -> Mat4;
    fn get_proj_transform(&self) -> Mat4;
}
