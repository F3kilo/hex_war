use crate::graphics::{LoadError, NotFoundError};
use glam::Mat4;
use std::cmp::Ordering;
use std::hash::Hash;
use std::rc::Rc;
use std::{fmt, hash};

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
struct Transforms {
    world: Mat4,
    view: Mat4,
    proj: Mat4,
}

pub type SharedManager = Rc<dyn ViewManager>;

pub struct View {
    id: ViewId,
    manager: SharedManager,
}

impl View {
    pub fn new(mut manager: SharedManager, transforms: Transforms) -> Result<Self, LoadError> {
        let id = manager.create_view(transforms)?;
        Ok(Self { id, manager })
    }

    fn get_world_transforms(&self) -> &Mat4 {
        self.manager.get_world_transforms(self.id).unwrap()
    }
    fn set_world_transforms(&mut self, transforms: Mat4) {
        self.manager
            .set_world_transforms(self.id, transforms)
            .unwrap()
    }
    fn get_view_transforms(&self) -> &Mat4 {
        self.manager.get_view_transforms(self.id).unwrap()
    }
    fn set_view_transforms(&mut self, transforms: Mat4) {
        self.manager
            .set_view_transforms(self.id, transforms)
            .unwrap()
    }
    fn get_proj_transforms(&self) -> &Mat4 {
        self.manager.get_proj_transforms(self.id).unwrap()
    }
    fn set_proj_transforms(&mut self, transforms: Mat4) {
        self.manager
            .set_proj_transforms(self.id, transforms)
            .unwrap()
    }

    pub fn get_id(&self) -> ViewId {
        self.id
    }
}

impl Drop for View {
    fn drop(&mut self) {
        self.manager.drop_view(self.id);
    }
}

impl fmt::Debug for View {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "View: id = {:?};", self.id,)
    }
}

impl PartialEq for View {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialEq<ViewId> for View {
    fn eq(&self, id: &ViewId) -> bool {
        self.id == *id
    }
}

impl Eq for View {}

impl Hash for View {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl PartialOrd for View {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl PartialOrd<ViewId> for View {
    fn partial_cmp(&self, id: &ViewId) -> Option<Ordering> {
        self.id.partial_cmp(&id)
    }
}

impl Ord for View {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub type SharedView = Rc<View>;
