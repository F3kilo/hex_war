use crate::graphics::error::{LoadError, NotFoundError};
use crate::graphics::manager::manage_textures::TextureId;
use crate::graphics::proxy::texture_manager::TextureManager;
use crate::math::screen_coords::ScreenCoords;
use std::fmt;
use std::path::PathBuf;
use std::rc::Rc;

pub struct UniqueTexture {
    id: TextureId,
    manager: TextureManager,
}

impl UniqueTexture {
    pub fn new(path: PathBuf, mut manager: TextureManager) -> Result<Self, LoadError> {
        let id = manager.create_texture(path)?;
        Ok(Self { manager, id })
    }

    pub fn from_raw(id: TextureId, manager: TextureManager) -> Result<Self, NotFoundError> {
        if !manager.contains(id) {
            return Err(NotFoundError);
        }
        Ok(Self { id, manager })
    }

    pub fn get_path(&self) -> PathBuf {
        self.manager
            .get_path(self.id)
            .expect("Texture id is not found, unexpectedly.")
    }

    pub fn get_size(&self) -> ScreenCoords {
        self.manager
            .get_size(self.id)
            .expect("Texture id is not found, unexpectedly.")
    }
}

impl Drop for UniqueTexture {
    fn drop(&mut self) {
        self.manager.drop_texture(self.id);
    }
}

impl fmt::Debug for UniqueTexture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Texture #{:?}", self.id)
    }
}

pub type Texture = Rc<UniqueTexture>;
