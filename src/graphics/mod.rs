use crate::graphics::backend::RenderBackend;
use crate::graphics::manager::geometry_manager::{CommonGeometryManager, GeometryManager};
use crate::graphics::manager::texture_manager::{CommonTextureManager, TextureId, TextureManager};
use crate::math::screen_coords::ScreenCoords;
use crate::math::world_coords::WorldCoords;
use glam::Mat4;
use std::error::Error;
use std::fmt;
use std::path::PathBuf;

pub mod backend;
pub mod manager;
pub mod primitive;

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

#[derive(Debug, Clone)]
pub struct PostEffect;

#[derive(Debug)]
pub struct RenderData;

#[derive(Debug)]
pub struct PresentInfo;

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

pub type Depth = f32;

pub trait Camera {
    fn to_world(&self, point: ScreenCoords, depth: Depth, screen_size: ScreenCoords)
        -> WorldCoords;
    fn to_screen(&self, world: WorldCoords, screen_size: ScreenCoords) -> (ScreenCoords, Depth);

    fn get_view_transform(&self) -> Mat4;
    fn get_proj_transform(&self) -> Mat4;
}

pub trait Render {}

pub trait Graphics {
    fn get_mut_texture_manager(&mut self) -> &mut dyn TextureManager;
    fn get_texture_manager(&self) -> &dyn TextureManager;

    fn get_mut_geometry_manager(&mut self) -> &mut dyn GeometryManager;
    fn get_geometry_manager(&self) -> &dyn GeometryManager;

    fn get_mut_renderer(&mut self) -> &mut dyn Render;
    fn get_renderer(&self) -> &dyn Render;

    fn present(&mut self, info: PresentInfo);
}

pub struct UniqueGraphics {
    backend: Box<dyn Graphics>,
}

impl UniqueGraphics {
    pub fn new(backend: Box<dyn Graphics>) -> Self {
        Self { backend }
    }
}

impl Graphics for UniqueGraphics {
    fn get_mut_texture_manager(&mut self) -> &mut dyn TextureManager {
        self.backend.get_mut_texture_manager()
    }

    fn get_texture_manager(&self) -> &dyn TextureManager {
        self.backend.get_texture_manager()
    }

    fn get_mut_geometry_manager(&mut self) -> &mut dyn GeometryManager {
        self.backend.get_mut_geometry_manager()
    }

    fn get_geometry_manager(&self) -> &dyn GeometryManager {
        self.backend.get_geometry_manager()
    }

    fn get_mut_renderer(&mut self) -> &mut dyn Render {
        self.backend.get_mut_renderer()
    }

    fn get_renderer(&self) -> &dyn Render {
        self.backend.get_renderer()
    }

    fn present(&mut self, info: PresentInfo) {
        self.backend.present(info)
    }
}
