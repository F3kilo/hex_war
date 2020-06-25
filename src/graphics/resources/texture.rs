use crate::graphics::{LoadError, NotFoundError};
use crate::math::screen_coords::ScreenCoords;
use std::borrow::BorrowMut;
use std::cell::{Ref, RefCell, RefMut};
use std::cmp::Ordering;
use std::hash::Hash;
use std::ops::{Deref, DerefMut};
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

pub type SharedManager = Rc<RefCell<dyn TextureManager>>;

pub struct UniqueTexture {
    id: TextureId,
    manager: SharedManager,
}

impl UniqueTexture {
    pub fn new(path: PathBuf, manager: SharedManager) -> Result<Self, LoadError> {
        let id = Self::get_mut_manager(&manager).create_texture(path)?;
        Ok(Self { id, manager })
    }

    pub fn get_size(&self) -> ScreenCoords {
        Self::get_manager(&self.manager).get_size(self.id).unwrap()
    }

    pub fn get_path(&self) -> PathBuf {
        Self::get_manager(&self.manager)
            .get_path(self.id)
            .unwrap()
            .to_path_buf()
    }

    pub fn get_id(&self) -> TextureId {
        self.id
    }

    fn get_manager(manager: &SharedManager) -> Ref<dyn TextureManager> {
        RefCell::try_borrow(&manager).expect("Can't get reference to texture manager.")
    }

    fn get_mut_manager(manager: &SharedManager) -> RefMut<dyn TextureManager> {
        RefCell::try_borrow_mut(&manager).expect("Can't get mutable reference to texture manager.")
    }
}

impl Drop for UniqueTexture {
    fn drop(&mut self) {
        Self::get_mut_manager(&self.manager).drop_texture(self.id);
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
