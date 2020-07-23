use crate::graphics::backend::GraphicsBackend;
use crate::graphics::error::{LoadError, UnavailableError};
use crate::graphics::manager::manage_textures::TextureId;
use crate::math::screen_coords::ScreenCoords;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

#[derive(Clone)]
pub struct TextureManager {
    backend: Rc<RefCell<Box<dyn GraphicsBackend>>>,
}

impl TextureManager {
    pub fn new(backend: Rc<RefCell<Box<dyn GraphicsBackend>>>) -> Self {
        Self { backend }
    }

    pub fn create_texture(&mut self, path: PathBuf) -> Result<TextureId, LoadError> {
        let mut back_ref = RefCell::borrow_mut(&self.backend);
        back_ref.get_mut_texture_manager().create_texture(path)
    }

    pub fn drop_texture(&mut self, id: TextureId) -> bool {
        let mut back_ref = RefCell::borrow_mut(&self.backend);
        back_ref.get_mut_texture_manager().drop_texture(id)
    }

    pub fn get_path(&self, id: TextureId) -> Result<PathBuf, UnavailableError> {
        let back_ref = RefCell::borrow(&self.backend);
        back_ref.get_texture_manager().get_path(id)
    }

    pub fn get_size(&self, id: TextureId) -> Result<ScreenCoords, UnavailableError> {
        let back_ref = RefCell::borrow(&self.backend);
        back_ref.get_texture_manager().get_size(id)
    }

    pub fn contains(&self, id: TextureId) -> bool {
        let back_ref = RefCell::borrow(&self.backend);
        back_ref.get_texture_manager().contains(id)
    }

    pub fn ids(&self) -> Vec<TextureId> {
        let back_ref = RefCell::borrow(&self.backend);
        back_ref.get_texture_manager().ids()
    }
}
