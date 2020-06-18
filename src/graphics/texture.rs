use crate::graphics::ivec::IVec2;
use std::cmp::Ordering;
use std::error::Error;
use std::hash::Hash;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::{fmt, hash};

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, Ord, PartialOrd)]
pub struct TextureId(u64);

pub trait TextureManager {
    fn load_texture(&mut self, path: PathBuf) -> Result<TextureId, LoadError>;
    fn drop_texture(&mut self, id: TextureId) -> bool;
    fn get_size(&self, id: TextureId) -> Result<IVec2, NotFoundError>;
    fn get_path(&self, id: TextureId) -> Result<&Path, NotFoundError>;
}

#[derive(Debug)]
pub enum LoadError {}

impl Error for LoadError {}
impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Can't load texture")
    }
}

#[derive(Debug)]
pub enum NotFoundError {}

impl Error for NotFoundError {}
impl fmt::Display for NotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Texture with specified id is not loaded.")
    }
}

type SharedManager = Rc<dyn TextureManager>;

pub struct Texture {
    id: TextureId,
    manager: SharedManager,
}

impl Texture {
    pub fn new(path: PathBuf, mut manager: SharedManager) -> Result<Self, LoadError> {
        let id = manager.load_texture(path)?;
        Ok(Self { id, manager })
    }

    pub fn get_size(&self) -> IVec2 {
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