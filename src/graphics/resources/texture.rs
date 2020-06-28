use crate::graphics::manager::texture_manager::{CommonTextureManager, TextureId, TextureManager};
use crate::graphics::LoadError;
use std::cmp::Ordering;
use std::hash::Hash;
use std::path::PathBuf;
use std::rc::Rc;
use std::{fmt, hash};

pub struct UniqueTexture {
    id: TextureId,
    manager: CommonTextureManager,
}

impl UniqueTexture {
    pub fn new(path: PathBuf, mut manager: CommonTextureManager) -> Result<Self, LoadError> {
        let id = manager.create_texture(path)?;
        Ok(Self { id, manager })
    }

    pub fn get_path(&self) -> PathBuf {
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
            "Geometry: id = {:?}; path = '{:?}'",
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
