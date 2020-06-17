pub mod sprite2d;
mod vk_backend;
use glam::Mat4;
use std::error::Error;
use std::fmt;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};

static RESOURCE_INDEX: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Mesh(u64);

impl Mesh {
    fn new() -> Self {
        let index = RESOURCE_INDEX.fetch_add(1, Ordering::Relaxed);
        Self(index)
    }
}

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Texture(u64);

impl Texture {
    pub fn new() -> Self {
        let index = RESOURCE_INDEX.fetch_add(1, Ordering::Relaxed);
        Self(index)
    }
}

pub struct MeshInfo {
    pub mesh: Mesh,
    pub pos_transforms: Mat4,
    pub uv_transforms: Mat4,
}

pub struct TextureInfo {}

pub trait Renderer {
    fn render_textured_mesh(&mut self, mesh: MeshInfo, texture: Texture);
}

#[derive(Debug)]
enum LoadError {}

impl Error for LoadError {}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Can't load resource")
    }
}

pub trait ResourceManager {
    fn load_mesh(&mut self, path: Path) -> Result<Mesh, LoadError>;
    fn load_texture(&mut self, path: Path) -> Result<Texture, LoadError>;
}

pub trait View {
    fn get_world(&self);
    fn set_world(&mut self);

    fn get_view(&self);
    fn set_view(&mut self);

    fn get_proj(&self);
    fn set_proj(&mut self);
}

pub struct Graphics {}

impl Graphics {}
