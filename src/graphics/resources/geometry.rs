use crate::graphics::{LoadError, NotFoundError};
use std::cell::{Ref, RefCell, RefMut};
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
        let id = Self::get_mut_manager(&manager).create_geometry(path)?;
        Ok(Self { id, manager })
    }

    pub fn get_path(&self) -> PathBuf {
        Self::get_manager(&self.manager)
            .get_path(self.id)
            .unwrap()
            .into()
    }

    pub fn get_id(&self) -> GeometryId {
        self.id
    }

    fn get_manager(manager: &SharedManager) -> Ref<dyn GeometryManager> {
        RefCell::try_borrow(&manager).expect("Can't get reference to geometry manager.")
    }

    fn get_mut_manager(manager: &SharedManager) -> RefMut<dyn GeometryManager> {
        RefCell::try_borrow_mut(&manager).expect("Can't get mutable reference to geometry manager.")
    }
}

impl Drop for UniqueGeometry {
    fn drop(&mut self) {
        Self::get_mut_manager(&self.manager).drop_geometry(self.id);
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
