use crate::graphics::{LoadError, NotFoundError};
use glam::Mat4;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::hash::Hash;
use std::rc::Rc;
use std::{fmt, hash};
use std::borrow::BorrowMut;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, Ord, PartialOrd)]
pub struct ViewId(u64);

pub trait ViewManager {
    fn create_view(&mut self, transforms: Transforms) -> Result<ViewId, LoadError>;
    fn drop_view(&mut self, id: ViewId) -> bool;

    fn get_world_transforms(&self, id: ViewId) -> Result<&Mat4, NotFoundError>;
    fn set_world_transforms(&mut self, id: ViewId, transforms: Mat4) -> Result<(), NotFoundError>;
    fn get_view_transforms(&self, id: ViewId) -> Result<&Mat4, NotFoundError>;
    fn set_view_transforms(&mut self, id: ViewId, transforms: Mat4) -> Result<(), NotFoundError>;
    fn get_proj_transforms(&self, id: ViewId) -> Result<&Mat4, NotFoundError>;
    fn set_proj_transforms(&mut self, id: ViewId, transforms: Mat4) -> Result<(), NotFoundError>;

    fn ids(&self) -> &dyn Iterator<Item = ViewId>;
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Transforms {
    world: Mat4,
    view: Mat4,
    proj: Mat4,
}

pub type SharedManager = Rc<dyn ViewManager>;

pub struct UniqueView {
    id: ViewId,
    manager: SharedManager,
}

impl UniqueView {
    pub fn new(mut manager: SharedManager, transforms: Transforms) -> Result<Self, LoadError> {
        let id = manager.create_view(transforms)?;
        Ok(Self { id, manager })
    }

    pub fn get_world_transforms(&self) -> &Mat4 {
        self.manager.get_world_transforms(self.id).unwrap()
    }
    pub fn set_world_transforms(&mut self, transforms: Mat4) {
        self.manager
            .set_world_transforms(self.id, transforms)
            .unwrap()
    }
    pub fn get_view_transforms(&self) -> &Mat4 {
        self.manager.get_view_transforms(self.id).unwrap()
    }
    pub fn set_view_transforms(&mut self, transforms: Mat4) {
        self.manager
            .set_view_transforms(self.id, transforms)
            .unwrap()
    }
    pub fn get_proj_transforms(&self) -> &Mat4 {
        self.manager.get_proj_transforms(self.id).unwrap()
    }
    pub fn set_proj_transforms(&mut self, transforms: Mat4) {
        self.manager
            .set_proj_transforms(self.id, transforms)
            .unwrap()
    }

    pub fn get_id(&self) -> ViewId {
        self.id
    }
}

impl Drop for UniqueView {
    fn drop(&mut self) {
        self.manager.drop_view(self.id);
    }
}

impl fmt::Debug for UniqueView {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "View: id = {:?};", self.id,)
    }
}

impl PartialEq for UniqueView {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialEq<ViewId> for UniqueView {
    fn eq(&self, id: &ViewId) -> bool {
        self.id == *id
    }
}

impl Eq for UniqueView {}

impl Hash for UniqueView {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl PartialOrd for UniqueView {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl PartialOrd<ViewId> for UniqueView {
    fn partial_cmp(&self, id: &ViewId) -> Option<Ordering> {
        self.id.partial_cmp(&id)
    }
}

impl Ord for UniqueView {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub type View = Rc<UniqueView>;
