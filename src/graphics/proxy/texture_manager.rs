use crate::graphics::error::{LoadError, NotFoundError};
use crate::graphics::low_level::Graphics;
use crate::graphics::manager::texture_manager::{TextureId, TextureManager};
use crate::math::screen_coords::ScreenCoords;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

#[derive(Clone)]
pub struct TextureManagerProxy {
    backend: Rc<RefCell<Box<dyn Graphics>>>,
}

impl TextureManagerProxy {
    pub fn new(backend: Rc<RefCell<Box<dyn Graphics>>>) -> Self {
        Self { backend }
    }
}

impl TextureManager for TextureManagerProxy {
    fn create_texture(&mut self, path: PathBuf) -> Result<TextureId, LoadError> {
        let mut back_ref = RefCell::borrow_mut(&self.backend);
        back_ref.get_mut_texture_manager().create_texture(path)
    }

    fn drop_texture(&mut self, id: TextureId) -> bool {
        let mut back_ref = RefCell::borrow_mut(&self.backend);
        back_ref.get_mut_texture_manager().drop_texture(id)
    }

    fn get_path(&self, id: TextureId) -> Result<PathBuf, NotFoundError> {
        let back_ref = RefCell::borrow(&self.backend);
        back_ref.get_texture_manager().get_path(id)
    }

    fn get_size(&self, id: TextureId) -> Result<ScreenCoords, NotFoundError> {
        let back_ref = RefCell::borrow(&self.backend);
        back_ref.get_texture_manager().get_size(id)
    }

    fn contains(&self, id: TextureId) -> bool {
        let back_ref = RefCell::borrow(&self.backend);
        back_ref.get_texture_manager().contains(id)
    }

    fn ids(&self) -> Vec<TextureId> {
        let back_ref = RefCell::borrow(&self.backend);
        back_ref.get_texture_manager().ids()
    }
}
