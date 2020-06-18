use crate::graphics::texture::SharedTexture;
use crate::graphics::{LoadError, NotFoundError};
use palette::Srgba;
use std::cmp::Ordering;
use std::hash::Hash;
use std::rc::Rc;
use std::{fmt, hash};

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, Ord, PartialOrd)]
pub struct MaterialId(u64);

pub trait MaterialManager {
    fn create_material(
        &mut self,
        color_map: SharedTexture,
        color: Srgba,
    ) -> Result<MaterialId, LoadError>;
    fn drop_material(&mut self, id: MaterialId) -> bool;

    fn set_color_map(&mut self, id: MaterialId, tex: SharedTexture) -> Result<(), NotFoundError>;
    fn get_color_map(&self, id: MaterialId) -> Result<&SharedTexture, NotFoundError>;

    fn set_color(&mut self, id: MaterialId, color: Srgba) -> Result<(), NotFoundError>;
    fn get_color(&self, id: MaterialId) -> Result<Srgba, NotFoundError>;
}

type SharedManager = Rc<dyn MaterialManager>;

pub struct Material {
    id: MaterialId,
    manager: SharedManager,
}

impl Material {
    pub fn new(
        mut manager: SharedManager,
        color_map: SharedTexture,
        color: Srgba,
    ) -> Result<Self, LoadError> {
        let id = manager.create_material(color_map, color)?;
        Ok(Self { id, manager })
    }

    fn set_color_map(&mut self, tex: SharedTexture) {
        self.manager.set_color_map(self.id, tex).unwrap()
    }

    fn get_color_map(&self) -> &SharedTexture {
        self.manager.get_color_map(self.id).unwrap()
    }

    fn set_color(&mut self, color: Srgba) {
        self.manager.set_color(self.id, color).unwrap()
    }

    fn get_color(&self) -> Srgba {
        self.manager.get_color(self.id).unwrap()
    }

    pub fn get_id(&self) -> MaterialId {
        self.id
    }
}

impl Drop for Material {
    fn drop(&mut self) {
        self.manager.drop_material(self.id);
    }
}

impl fmt::Debug for Material {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Material: id = {:?}; color_map = '{:?}'",
            self.id,
            self.get_color_map()
        )
    }
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialEq<MaterialId> for Material {
    fn eq(&self, id: &MaterialId) -> bool {
        self.id == *id
    }
}

impl Eq for Material {}

impl Hash for Material {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl PartialOrd for Material {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl PartialOrd<MaterialId> for Material {
    fn partial_cmp(&self, id: &MaterialId) -> Option<Ordering> {
        self.id.partial_cmp(&id)
    }
}

impl Ord for Material {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub type SharedMaterial = Rc<Material>;
