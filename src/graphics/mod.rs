use crate::graphics::resources::view::SharedView;
use crate::graphics::resources::{geometry, material, texture, view, Resources};
use glam::Mat4;
use std::error::Error;
use std::fmt;

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
    fn get_view_manager(&self) -> &view::SharedManager;
    fn get_material_manager(&self) -> &material::SharedManager;
    fn get_texture_manager(&self) -> &texture::SharedManager;
    fn get_geometry_manager(&self) -> &geometry::SharedManager;

    fn render(&mut self, resources: &Resources, transforms: &Transforms);
    fn apply_view(&mut self, view: SharedView);
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

impl Renderer {
    fn new(backend: Box<dyn RenderBackend>) -> Self {
        Self { backend }
    }

    fn render(&mut self, resources: &Resources, transforms: &Transforms) {}
}
