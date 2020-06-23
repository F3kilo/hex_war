use crate::graphics::resources::texture::Texture;
use crate::graphics::resources::view::View;
use crate::graphics::resources::{geometry, material, texture, view, Resources};
use glam::Mat4;
use palette::Srgba;
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
    fn create_view(&self, transforms: view::Transforms) -> view::View;
    fn create_material(&self, color_map: texture::Texture, color: Srgba) -> material::Material;
    fn create_texture(&self, path: PathBuf) -> texture::Texture;
    fn create_geometry(&self, path: PathBuf) -> geometry::Geometry;

    fn render(&mut self, resources: Resources, transforms: &Transforms);
    fn apply_view(&mut self, view: view::View);
    fn present(&mut self);
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Transforms {
    pub instance: Mat4,
    pub uv: Mat4,
}

pub struct Renderer {
    backend: Box<dyn RenderBackend>,
}

struct PostEffect;

impl Renderer {
    fn new(backend: Box<dyn RenderBackend>) -> Self {
        Self { backend }
    }

    fn render(&mut self, view: &View, post_effects: &impl Iterator<Item = PostEffect>) -> Texture {}
    fn present(
        &mut self,
        images: &impl Iterator<Item = Texture>,
        post_effects: &impl Iterator<Item = PostEffect>,
    ) {
    }
}
