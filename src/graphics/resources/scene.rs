use crate::graphics::resources::geometry::Geometry;
use crate::graphics::resources::texture::Texture;
use crate::graphics::{LoadError, NotFoundError};
use glam::{Mat4, Vec2};
use std::cell::{Ref, RefCell, RefMut};
use std::cmp::Ordering;
use std::hash::Hash;
use std::rc::Rc;
use std::{fmt, hash};

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, Ord, PartialOrd)]
pub struct SceneId(u64);

#[derive(Debug, Copy, Clone, Default)]
pub struct UvTransforms {
    pub offset: Vec2,
    pub scale: Vec2,
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Instance {
    pub transforms: Mat4,
    pub uv_transforms: UvTransforms,
}

#[derive(Debug, Clone)]
pub struct TexturedGeometry {
    geometry: Geometry,
    texture: Texture,
    instance: Instance,
}

pub trait SceneManager {
    fn create_scene(&mut self) -> Result<SceneId, LoadError>;
    fn drop_scene(&mut self, id: SceneId) -> bool;

    fn add_textured_geometry(
        &mut self,
        id: SceneId,
        inst: TexturedGeometry,
    ) -> Result<(), NotFoundError>;

    fn ids(&self) -> &dyn Iterator<Item = SceneId>;
}

pub type SharedManager = Rc<RefCell<dyn SceneManager>>;

pub struct Scene {
    id: SceneId,
    manager: SharedManager,
}

impl Scene {
    pub fn new(manager: SharedManager) -> Result<Self, LoadError> {
        let id = Self::get_mut_manager(&manager).create_scene()?;
        Ok(Self { id, manager })
    }

    pub fn add_textured_geometry(&mut self, inst: TexturedGeometry) {
        Self::get_mut_manager(&self.manager)
            .add_textured_geometry(self.id, inst)
            .unwrap()
    }

    pub fn get_id(&self) -> SceneId {
        self.id
    }

    fn get_manager(manager: &SharedManager) -> Ref<dyn SceneManager> {
        RefCell::try_borrow(&manager).expect("Can't get reference to texture manager.")
    }

    fn get_mut_manager(manager: &SharedManager) -> RefMut<dyn SceneManager> {
        RefCell::try_borrow_mut(&manager).expect("Can't get mutable reference to texture manager.")
    }
}

impl Drop for Scene {
    fn drop(&mut self) {
        Self::get_mut_manager(&self.manager).drop_scene(self.id);
    }
}

impl fmt::Debug for Scene {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Scene: id = {:?};", self.id,)
    }
}

impl PartialEq for Scene {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialEq<SceneId> for Scene {
    fn eq(&self, id: &SceneId) -> bool {
        self.id == *id
    }
}

impl Eq for Scene {}

impl Hash for Scene {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl PartialOrd for Scene {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl PartialOrd<SceneId> for Scene {
    fn partial_cmp(&self, id: &SceneId) -> Option<Ordering> {
        self.id.partial_cmp(&id)
    }
}

impl Ord for Scene {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
