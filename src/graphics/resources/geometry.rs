use crate::graphics::{LoadError, NotFoundError};
use std::cell::RefCell;
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

    fn ids(&self) -> &dyn Iterator<Item = GeometryId>;
}

pub type SharedManager = Rc<RefCell<dyn GeometryManager>>;

pub struct UniqueGeometry {
    id: GeometryId,
    manager: SharedManager,
}

impl UniqueGeometry {
    pub fn new(path: PathBuf, manager: SharedManager) -> Result<Self, LoadError> {
        let id = manager.borrow_mut().create_geometry(path)?;
        Ok(Self { id, manager })
    }

    pub fn get_path(&self) -> PathBuf {
        self.manager.borrow().get_path(self.id).unwrap().into()
    }

    pub fn get_id(&self) -> GeometryId {
        self.id
    }
}

impl Drop for UniqueGeometry {
    fn drop(&mut self) {
        self.manager.borrow_mut().drop_geometry(self.id);
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
