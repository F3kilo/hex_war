use crate::graphics::error::{LoadError, UnavailableError};
use crate::graphics::manager::manage_geometries::GeometryId;
use crate::graphics::proxy::geometry_manager::GeometryManager;
use std::fmt;
use std::hash::{Hash, Hasher};
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

    pub fn from_raw(id: GeometryId, manager: GeometryManager) -> Result<Self, UnavailableError> {
        if !manager.contains(id) {
            return Err(UnavailableError::NotFound);
        }
        Ok(Self { id, manager })
    }

    pub fn get_id(&self) -> GeometryId {
        self.id
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

impl PartialEq for UniqueGeometry {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for UniqueGeometry {}

impl Hash for UniqueGeometry {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash(&self.id, state)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Geometry {
    unique: Rc<UniqueGeometry>,
}

impl Geometry {
    pub fn new(path: PathBuf, manager: GeometryManager) -> Result<Self, LoadError> {
        UniqueGeometry::new(path, manager).map(|unique| Self {
            unique: Rc::new(unique),
        })
    }

    pub fn from_raw(id: GeometryId, manager: GeometryManager) -> Result<Self, UnavailableError> {
        UniqueGeometry::from_raw(id, manager).map(|unique| Self {
            unique: Rc::new(unique),
        })
    }

    pub fn get_id(&self) -> GeometryId {
        self.unique.get_id()
    }

    pub fn get_path(&self) -> PathBuf {
        self.unique.get_path()
    }
}
