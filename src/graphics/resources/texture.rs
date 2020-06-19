use crate::graphics::{LoadError, NotFoundError};
use crate::math::screen_coords::ScreenCoords;
use std::cmp::Ordering;
use std::hash::Hash;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::{fmt, hash};

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, Ord, PartialOrd)]
pub struct TextureId(u64);

pub trait TextureManager {
    fn create_texture(&mut self, path: PathBuf) -> Result<TextureId, LoadError>;
    fn drop_texture(&mut self, id: TextureId) -> bool;
    fn get_size(&self, id: TextureId) -> Result<ScreenCoords, NotFoundError>;
    fn get_path(&self, id: TextureId) -> Result<&Path, NotFoundError>;

    fn ids(&self) -> &dyn Iterator<Item = TextureId>;
}

pub type SharedManager = Rc<dyn TextureManager>;

pub struct Texture {
    id: TextureId,
    manager: SharedManager,
}

impl Texture {
    pub fn new(path: PathBuf, mut manager: SharedManager) -> Result<Self, LoadError> {
        let id = manager.create_texture(path)?;
        Ok(Self { id, manager })
    }

    pub fn get_size(&self) -> ScreenCoords {
        self.manager.get_size(self.id).unwrap()
    }

    pub fn get_path(&self) -> &Path {
        self.manager.get_path(self.id).unwrap()
    }

    pub fn get_id(&self) -> TextureId {
        self.id
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        self.manager.drop_texture(self.id);
    }
}

impl fmt::Debug for Texture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Texture: id = {:?}; path = '{:?}'",
            self.id,
            self.get_path()
        )
    }
}

impl PartialEq for Texture {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialEq<TextureId> for Texture {
    fn eq(&self, id: &TextureId) -> bool {
        self.id == *id
    }
}

impl Eq for Texture {}

impl Hash for Texture {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl PartialOrd for Texture {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl PartialOrd<TextureId> for Texture {
    fn partial_cmp(&self, id: &TextureId) -> Option<Ordering> {
        self.id.partial_cmp(&id)
    }
}

impl Ord for Texture {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub type SharedTexture = Rc<Texture>;
