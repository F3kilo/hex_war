use crate::graphics::error::{LoadError, NotFoundError};
use crate::graphics::manager::manage_textures::{ManageTextures, TextureId};
use crate::math::screen_coords::ScreenCoords;
use slog::Logger;
use std::path::PathBuf;

#[derive(Debug)]
pub struct VkTextureManager {
    logger: Logger,
}

impl VkTextureManager {
    pub fn new(logger: Logger) -> Self {
        Self { logger }
    }
}

impl ManageTextures for VkTextureManager {
    fn create_texture(&mut self, path: PathBuf) -> Result<TextureId, LoadError> {
        todo!()
    }

    fn drop_texture(&mut self, id: TextureId) -> bool {
        todo!()
    }

    fn get_path(&self, id: TextureId) -> Result<PathBuf, NotFoundError> {
        todo!()
    }

    fn get_size(&self, id: TextureId) -> Result<ScreenCoords, NotFoundError> {
        todo!()
    }

    fn contains(&self, id: TextureId) -> bool {
        todo!()
    }

    fn ids(&self) -> Vec<TextureId> {
        todo!()
    }
}
