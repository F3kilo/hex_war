pub mod vulkan;

use crate::graphics::resources::{geometry, scene, texture};
use crate::graphics::LoadError;
use std::path::PathBuf;

pub trait RenderBackend {
    fn create_texture(&self, path: PathBuf) -> Result<texture::Texture, LoadError>;
    fn create_geometry(&self, path: PathBuf) -> Result<geometry::Geometry, LoadError>;
    fn create_scene(&self) -> scene::Scene;
}
