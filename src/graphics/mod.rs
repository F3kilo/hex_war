use crate::graphics::resources::geometry::SharedGeometry;
use crate::graphics::resources::material::SharedMaterial;
use crate::graphics::resources::view::SharedView;
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

#[derive(Debug, Clone, Eq, PartialEq)]
struct Resources {
    pub material: SharedMaterial,
    pub geometry: SharedGeometry,
    pub view: SharedView,
}

pub struct RenderBackend {}

#[derive(Debug, Copy, Clone, Default)]
pub struct Transforms {
    pub instance: Mat4,
    pub uv: Mat4,
}

pub struct Renderer {}

impl Renderer {
    fn new(backend: RenderBackend) -> Self {}
}
