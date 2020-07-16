use crate::graphics::error::{LoadError, NotFoundError};
use crate::math::screen_coords::ScreenCoords;
use std::path::PathBuf;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, Ord, PartialOrd)]
pub struct TextureId(u64);

impl From<u64> for TextureId {
    fn from(v: u64) -> Self {
        Self(v)
    }
}

pub trait ManageTextures {
    fn create_texture(&mut self, path: PathBuf) -> Result<TextureId, LoadError>;
    fn drop_texture(&mut self, id: TextureId) -> bool;
    fn get_path(&self, id: TextureId) -> Result<PathBuf, NotFoundError>;
    fn get_size(&self, id: TextureId) -> Result<ScreenCoords, NotFoundError>;

    fn contains(&self, id: TextureId) -> bool;
    fn ids(&self) -> Vec<TextureId>;
}
