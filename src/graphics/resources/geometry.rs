use crate::graphics::{LoadError, NotFoundError};
use std::cmp::Ordering;
use std::hash::Hash;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::{fmt, hash};

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, Ord, PartialOrd)]
pub struct GeometryId(u64);

pub trait GeometryManager {
    fn create_geometry(&mut self, path: PathBuf) -> Result<GeometryId, LoadError>;
    fn drop_geometry(&mut self, id: GeometryId) -> bool;
    fn get_path(&self, id: GeometryId) -> Result<&Path, NotFoundError>;
}

type SharedManager = Rc<dyn GeometryManager>;

pub struct Geometry {
    id: GeometryId,
    manager: SharedManager,
}

impl Geometry {
    pub fn new(path: PathBuf, mut manager: SharedManager) -> Result<Self, LoadError> {
        let id = manager.create_geometry(path)?;
        Ok(Self { id, manager })
    }

    pub fn get_path(&self) -> &Path {
        self.manager.get_path(self.id).unwrap()
    }

    pub fn get_id(&self) -> GeometryId {
        self.id
    }
}

impl Drop for Geometry {
    fn drop(&mut self) {
        self.manager.drop_geometry(self.id);
    }
}

impl fmt::Debug for Geometry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Geometry: id = {:?}; path = '{:?}'",
            self.id,
            self.get_path()
        )
    }
}

impl PartialEq for Geometry {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialEq<GeometryId> for Geometry {
    fn eq(&self, id: &GeometryId) -> bool {
        self.id == *id
    }
}

impl Eq for Geometry {}

impl Hash for Geometry {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl PartialOrd for Geometry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl PartialOrd<GeometryId> for Geometry {
    fn partial_cmp(&self, id: &GeometryId) -> Option<Ordering> {
        self.id.partial_cmp(&id)
    }
}

impl Ord for Geometry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub type SharedGeometry = Rc<Geometry>;
