use crate::graphics::error::LoadError;
use crate::graphics::manager::manage_geometries::GeometryId;
use crate::graphics::proxy::geometry_manager::GeometryManager;
use std::fmt;
use std::path::PathBuf;
use std::rc::Rc;

pub struct UniqueGeometry {
    id: GeometryId,
    manager: GeometryManager,
}

impl UniqueGeometry {
    pub fn new(path: PathBuf, mut manager: GeometryManager) -> Result<Self, LoadError> {
        let id = manager.create_geometry(path)?;
        Ok(Self { id, manager })
    }

    pub fn get_path(&self) -> PathBuf {
        self.manager
            .get_path(self.id)
            .expect("Geometry id is not found, unexpectedly.")
    }
}

impl Drop for UniqueGeometry {
    fn drop(&mut self) {
        self.manager.drop_geometry(self.id);
    }
}

impl fmt::Debug for UniqueGeometry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Geometry #{:?}", self.id)
    }
}

pub type Geometry = Rc<UniqueGeometry>;
