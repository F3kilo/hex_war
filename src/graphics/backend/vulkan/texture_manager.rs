use crate::graphics::manager::texture_manager::{TextureId, TextureManager};
use crate::graphics::{LoadError, NotFoundError};
use crate::math::screen_coords::ScreenCoords;
use std::path::PathBuf;

#[derive(Debug)]
pub struct VkTextureManager {}

impl TextureManager for VkTextureManager {
    fn create_texture(&mut self, path: PathBuf) -> Result<TextureId, LoadError> {
        unimplemented!()
    }

    fn drop_texture(&mut self, id: TextureId) -> bool {
        unimplemented!()
    }

    fn get_path(&self, id: TextureId) -> Result<PathBuf, NotFoundError> {
        unimplemented!()
    }

    fn get_size(&self, id: TextureId) -> Result<ScreenCoords, NotFoundError> {
        unimplemented!()
    }

    fn contains(&self, id: TextureId) -> bool {
        unimplemented!()
    }

    fn ids(&self) -> Vec<TextureId> {
        unimplemented!()
    }
}
