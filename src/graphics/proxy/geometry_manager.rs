use crate::graphics::error::{LoadError, NotFoundError};
use crate::graphics::low_level::GraphicsBackend;
use crate::graphics::manager::geometry_manager::GeometryId;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

#[derive(Clone)]
pub struct GeometryManager {
    backend: Rc<RefCell<Box<dyn GraphicsBackend>>>,
}

impl GeometryManager {
    pub fn new(backend: Rc<RefCell<Box<dyn GraphicsBackend>>>) -> Self {
        Self { backend }
    }

    pub fn create_geometry(&mut self, path: PathBuf) -> Result<GeometryId, LoadError> {
        let mut back_ref = RefCell::borrow_mut(&self.backend);
        back_ref.get_mut_geometry_manager().create_geometry(path)
    }

    pub fn drop_geometry(&mut self, id: GeometryId) -> bool {
        let mut back_ref = RefCell::borrow_mut(&self.backend);
        back_ref.get_mut_geometry_manager().drop_geometry(id)
    }

    pub fn get_path(&self, id: GeometryId) -> Result<PathBuf, NotFoundError> {
        let back_ref = RefCell::borrow(&self.backend);
        back_ref.get_geometry_manager().get_path(id)
    }

    pub fn contains(&self, id: GeometryId) -> bool {
        let back_ref = RefCell::borrow(&self.backend);
        back_ref.get_geometry_manager().contains(id)
    }

    pub fn ids(&self) -> Vec<GeometryId> {
        let back_ref = RefCell::borrow(&self.backend);
        back_ref.get_geometry_manager().ids()
    }
}
