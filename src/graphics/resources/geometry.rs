use crate::graphics::manager::geometry_manager::{
    CommonGeometryManager, GeometryId, GeometryManager,
};
use crate::graphics::LoadError;
use std::cmp::Ordering;
use std::hash::Hash;
use std::path::PathBuf;
use std::rc::Rc;
use std::{fmt, hash};

pub struct UniqueGeometry {
    id: GeometryId,
    manager: CommonGeometryManager,
}

impl UniqueGeometry {
    pub fn new(path: PathBuf, mut manager: CommonGeometryManager) -> Result<Self, LoadError> {
        let id = manager.create_geometry(path)?;
        Ok(Self { id, manager })
    }

    pub fn get_path(&self) -> PathBuf {
        self.manager.get_path(self.id).unwrap()
    }

    pub fn get_id(&self) -> GeometryId {
        self.id
    }
}

impl Drop for UniqueGeometry {
    fn drop(&mut self) {
        self.manager.drop_geometry(self.id);
    }
}

impl fmt::Debug for UniqueGeometry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Geometry: id = {:?}; path = '{:?}'",
            self.id,
            self.get_path()
        )
    }
}

impl PartialEq for UniqueGeometry {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialEq<GeometryId> for UniqueGeometry {
    fn eq(&self, id: &GeometryId) -> bool {
        self.id == *id
    }
}

impl Eq for UniqueGeometry {}

impl Hash for UniqueGeometry {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl PartialOrd for UniqueGeometry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl PartialOrd<GeometryId> for UniqueGeometry {
    fn partial_cmp(&self, id: &GeometryId) -> Option<Ordering> {
        self.id.partial_cmp(&id)
    }
}

impl Ord for UniqueGeometry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub type Geometry = Rc<UniqueGeometry>;
