use crate::graphics::resources::texture::Texture;
use crate::graphics::{LoadError, NotFoundError};
use palette::Srgba;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::hash::Hash;
use std::rc::Rc;
use std::{fmt, hash};

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, Ord, PartialOrd)]
pub struct MaterialId(u64);

pub trait MaterialManager {
    fn create_material(
        &mut self,
        color_map: Texture,
        color: Srgba,
    ) -> Result<MaterialId, LoadError>;
    fn drop_material(&mut self, id: MaterialId) -> bool;

    fn set_color_map(&mut self, id: MaterialId, tex: Texture) -> Result<(), NotFoundError>;
    fn get_color_map(&self, id: MaterialId) -> Result<&Texture, NotFoundError>;

    fn set_color(&mut self, id: MaterialId, color: Srgba) -> Result<(), NotFoundError>;
    fn get_color(&self, id: MaterialId) -> Result<Srgba, NotFoundError>;

    fn ids(&self) -> &dyn Iterator<Item = MaterialId>;
}

pub type SharedManager = Rc<RefCell<dyn MaterialManager>>;

pub struct UniqueMaterial {
    id: MaterialId,
    manager: SharedManager,
}

impl UniqueMaterial {
    pub fn new(
        manager: SharedManager,
        color_map: Texture,
        color: Srgba,
    ) -> Result<Self, LoadError> {
        let id = manager.borrow_mut().create_material(color_map, color)?;
        Ok(Self { id, manager })
    }

    fn set_color_map(&mut self, tex: Texture) {
        self.manager.set_color_map(self.id, tex).unwrap()
    }

    fn get_color_map(&self) -> &Texture {
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

impl Drop for UniqueMaterial {
    fn drop(&mut self) {
        self.manager.drop_material(self.id);
    }
}

impl fmt::Debug for UniqueMaterial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Material: id = {:?}; color_map = '{:?}'",
            self.id,
            self.get_color_map()
        )
    }
}

impl PartialEq for UniqueMaterial {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialEq<MaterialId> for UniqueMaterial {
    fn eq(&self, id: &MaterialId) -> bool {
        self.id == *id
    }
}

impl Eq for UniqueMaterial {}

impl Hash for UniqueMaterial {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl PartialOrd for UniqueMaterial {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl PartialOrd<MaterialId> for UniqueMaterial {
    fn partial_cmp(&self, id: &MaterialId) -> Option<Ordering> {
        self.id.partial_cmp(&id)
    }
}

impl Ord for UniqueMaterial {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub type Material = Rc<UniqueMaterial>;
