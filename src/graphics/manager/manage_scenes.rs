use crate::graphics::error::LoadError;
use crate::graphics::manager::manage_geometries::GeometryId;
use crate::graphics::manager::manage_textures::TextureId;
use glam::{Mat4, Vec2};
use palette::Srgba;
use std::error::Error;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, Ord, PartialOrd)]
pub struct SceneId(u64);

pub trait ManageScenes {
    fn create_scene(&mut self) -> Result<SceneId, LoadError>;
    fn drop_scene(&mut self, id: SceneId) -> bool;

    fn add_textured_geometry(
        &mut self,
        id: SceneId,
        instance: TexturedGeometry,
    ) -> Result<(), AdditionError>;
    
    fn render(&mut self, id: SceneId, context: &RenderContext) -> TextureId;

    fn contains(&self, id: SceneId) -> bool;
    fn ids(&self) -> Vec<SceneId>;
}

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
    pub geometry: GeometryId,
    pub texture: TextureId,
    pub instance: Instance,
}

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

#[derive(Debug)]
pub enum AdditionError {
    SceneNotFound(SceneId),
    TextureNotFound(TextureId),
    GeometryNotFound(GeometryId),
}

impl AdditionError {
    pub fn texture_not_found(tex_id: TextureId) -> Self {
        AdditionError::TextureNotFound(tex_id)
    }

    pub fn geometry_not_found(tex_id: TextureId) -> Self {
        AdditionError::TextureNotFound(tex_id)
    }
}

impl Error for AdditionError {}

impl fmt::Display for AdditionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AdditionError::SceneNotFound(id) => write!(f, "Scene with id: {:?} not found.", id),
            AdditionError::TextureNotFound(id) => write!(f, "Texture with id: {:?} not found.", id),
            AdditionError::GeometryNotFound(id) => {
                write!(f, "Geometry with id: {:?} not found.", id)
            }
        }
    }
}
