use crate::graphics::{LoadError, NotFoundError};
use std::cell::{Ref, RefCell, RefMut};
use std::fmt;
use std::path::PathBuf;
use std::rc::Rc;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, Ord, PartialOrd)]
pub struct GeometryId(u64);

pub trait GeometryManager: fmt::Debug {
    fn create_geometry(&mut self, path: PathBuf) -> Result<GeometryId, LoadError>;
    fn drop_geometry(&mut self, id: GeometryId) -> bool;
    fn get_path(&self, id: GeometryId) -> Result<PathBuf, NotFoundError>;

    fn ids(&self) -> Vec<GeometryId>;
}

pub type SharedGeometryManager = Rc<RefCell<dyn GeometryManager>>;

#[derive(Clone, Debug)]
pub struct CommonGeometryManager {
    backend: SharedGeometryManager,
}

impl CommonGeometryManager {
    pub fn new(backend: Rc<RefCell<dyn GeometryManager>>) -> Self {
        Self { backend }
    }

    fn get_mut_backend(&mut self) -> RefMut<dyn GeometryManager> {
        RefCell::try_borrow_mut(&self.backend)
            .expect("Can't get mutable reference to geometry manager.")
    }

    fn get_backend(&self) -> Ref<dyn GeometryManager> {
        RefCell::try_borrow(&self.backend).expect("Can't get reference to geometry manager.")
    }
}

impl GeometryManager for CommonGeometryManager {
    fn create_geometry(&mut self, path: PathBuf) -> Result<GeometryId, LoadError> {
        self.get_mut_backend().create_geometry(path)
    }
    fn drop_geometry(&mut self, id: GeometryId) -> bool {
        self.get_mut_backend().drop_geometry(id)
    }
    fn get_path(&self, id: GeometryId) -> Result<PathBuf, NotFoundError> {
        self.get_backend().get_path(id)
    }

    fn ids(&self) -> Vec<GeometryId> {
        self.get_backend().ids()
    }
}
