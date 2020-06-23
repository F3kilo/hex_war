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

pub struct UniqueTexture {
    id: TextureId,
    manager: SharedManager,
}

impl UniqueTexture {
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

impl Drop for UniqueTexture {
    fn drop(&mut self) {
        self.manager.drop_texture(self.id);
    }
}

impl fmt::Debug for UniqueTexture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Texture: id = {:?}; path = '{:?}'",
            self.id,
            self.get_path()
        )
    }
}

impl PartialEq for UniqueTexture {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialEq<TextureId> for UniqueTexture {
    fn eq(&self, id: &TextureId) -> bool {
        self.id == *id
    }
}

impl Eq for UniqueTexture {}

impl Hash for UniqueTexture {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl PartialOrd for UniqueTexture {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl PartialOrd<TextureId> for UniqueTexture {
    fn partial_cmp(&self, id: &TextureId) -> Option<Ordering> {
        self.id.partial_cmp(&id)
    }
}

impl Ord for UniqueTexture {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub type Texture = Rc<UniqueTexture>;
