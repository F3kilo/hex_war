use crate::graphics::{LoadError, NotFoundError};
use crate::math::screen_coords::ScreenCoords;
use std::cell::{Ref, RefCell, RefMut};
use std::fmt;
use std::path::PathBuf;
use std::rc::Rc;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, Ord, PartialOrd)]
pub struct TextureId(u64);

pub trait TextureManager: fmt::Debug {
    fn create_texture(&mut self, path: PathBuf) -> Result<TextureId, LoadError>;
    fn drop_texture(&mut self, id: TextureId) -> bool;
    fn get_path(&self, id: TextureId) -> Result<PathBuf, NotFoundError>;
    fn get_size(&self, id: TextureId) -> Result<ScreenCoords, NotFoundError>;

    fn ids(&self) -> Vec<TextureId>;
}

pub type SharedTextureManager = Rc<RefCell<dyn TextureManager>>;

#[derive(Clone, Debug)]
pub struct CommonTextureManager {
    backend: SharedTextureManager,
}

impl CommonTextureManager {
    pub fn new(backend: Rc<RefCell<dyn TextureManager>>) -> Self {
        Self { backend }
    }

    fn get_mut_backend(&mut self) -> RefMut<dyn TextureManager> {
        RefCell::try_borrow_mut(&self.backend)
            .expect("Can't get mutable reference to texture manager.")
    }

    fn get_backend(&self) -> Ref<dyn TextureManager> {
        RefCell::try_borrow(&self.backend).expect("Can't get reference to texture manager.")
    }
}

impl TextureManager for CommonTextureManager {
    fn create_texture(&mut self, path: PathBuf) -> Result<TextureId, LoadError> {
        self.get_mut_backend().create_texture(path)
    }
    fn drop_texture(&mut self, id: TextureId) -> bool {
        self.get_mut_backend().drop_texture(id)
    }
    fn get_path(&self, id: TextureId) -> Result<PathBuf, NotFoundError> {
        self.get_backend().get_path(id)
    }

    fn get_size(&self, id: TextureId) -> Result<ScreenCoords, NotFoundError> {
        self.get_backend().get_size(id)
    }

    fn ids(&self) -> Vec<TextureId> {
        self.get_backend().ids()
    }
}
