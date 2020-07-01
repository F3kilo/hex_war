use crate::graphics::error::{LoadError, NotFoundError};
use crate::graphics::low_level::Graphics;
use crate::graphics::manager::geometry_manager::{GeometryId, GeometryManager};
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

#[derive(Clone)]
pub struct GeometryManagerProxy {
    backend: Rc<RefCell<Box<dyn Graphics>>>,
}

impl GeometryManagerProxy {
    pub fn new(backend: Rc<RefCell<Box<dyn Graphics>>>) -> Self {
        Self { backend }
    }
}

impl GeometryManager for GeometryManagerProxy {
    fn create_geometry(&mut self, path: PathBuf) -> Result<GeometryId, LoadError> {
        let mut back_ref = RefCell::borrow_mut(&self.backend);
        back_ref.get_mut_geometry_manager().create_geometry(path)
    }

    fn drop_geometry(&mut self, id: GeometryId) -> bool {
        let mut back_ref = RefCell::borrow_mut(&self.backend);
        back_ref.get_mut_geometry_manager().drop_geometry(id)
    }

    fn get_path(&self, id: GeometryId) -> Result<PathBuf, NotFoundError> {
        let back_ref = RefCell::borrow(&self.backend);
        back_ref.get_geometry_manager().get_path(id)
    }

    fn contains(&self, id: GeometryId) -> bool {
        let back_ref = RefCell::borrow(&self.backend);
        back_ref.get_geometry_manager().contains(id)
    }

    fn ids(&self) -> Vec<GeometryId> {
        let back_ref = RefCell::borrow(&self.backend);
        back_ref.get_geometry_manager().ids()
    }
}
