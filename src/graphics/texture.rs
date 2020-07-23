use crate::graphics::error::{LoadError, UnavailableError};
use crate::graphics::manager::manage_textures::TextureId;
use crate::graphics::proxy::texture_manager::TextureManager;
use crate::math::screen_coords::ScreenCoords;
use std::fmt;
use std::hash::{Hash, Hasher};
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

    pub fn from_raw(id: TextureId, manager: TextureManager) -> Result<Self, UnavailableError> {
        if !manager.contains(id) {
            return Err(UnavailableError::NotFound);
        }
        Ok(Self { id, manager })
    }

    pub fn get_id(&self) -> TextureId {
        self.id
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

impl PartialEq for UniqueTexture {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for UniqueTexture {}

impl Hash for UniqueTexture {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash(&self.id, state)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Texture {
    unique: Rc<UniqueTexture>,
}

impl Texture {
    pub fn new(path: PathBuf, manager: TextureManager) -> Result<Self, LoadError> {
        UniqueTexture::new(path, manager).map(|unique| Self {
            unique: Rc::new(unique),
        })
    }

    pub fn get_id(&self) -> TextureId {
        self.unique.get_id()
    }

    pub fn from_raw(id: TextureId, manager: TextureManager) -> Result<Self, UnavailableError> {
        UniqueTexture::from_raw(id, manager).map(|unique| Self {
            unique: Rc::new(unique),
        })
    }

    pub fn get_path(&self) -> PathBuf {
        self.unique.get_path()
    }

    pub fn get_size(&self) -> ScreenCoords {
        self.unique.get_size()
    }
}
