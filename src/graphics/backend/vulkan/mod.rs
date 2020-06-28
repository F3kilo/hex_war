use crate::graphics::backend::RenderBackend;
use crate::graphics::manager::geometry_manager::{
    GeometryId, GeometryManager, SharedGeometryManager,
};
use crate::graphics::manager::texture_manager::{SharedTextureManager, TextureId, TextureManager};
use crate::graphics::{LoadError, NotFoundError};
use crate::math::screen_coords::ScreenCoords;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

#[derive(Debug)]
struct VkTextureManager {}

impl VkTextureManager {
    pub fn new() -> Self {
        Self {}
    }
}

impl TextureManager for VkTextureManager {
    fn create_texture(&mut self, path: PathBuf) -> Result<TextureId, LoadError> {
        unimplemented!()
    }

    fn drop_texture(&mut self, id: TextureId) -> bool {
        unimplemented!()
    }

    fn get_path(&self, id: TextureId) -> Result<PathBuf, NotFoundError> {
        unimplemented!()
    }

    fn get_size(&self, id: TextureId) -> Result<ScreenCoords, NotFoundError> {
        unimplemented!()
    }

    fn ids(&self) -> Vec<TextureId> {
        unimplemented!()
    }
}

#[derive(Debug)]
struct VkGeometryManager {}

impl VkGeometryManager {
    pub fn new() -> Self {
        Self {}
    }
}

impl GeometryManager for VkGeometryManager {
    fn create_geometry(&mut self, path: PathBuf) -> Result<GeometryId, LoadError> {
        unimplemented!()
    }

    fn drop_geometry(&mut self, id: GeometryId) -> bool {
        unimplemented!()
    }

    fn get_path(&self, id: GeometryId) -> Result<PathBuf, NotFoundError> {
        unimplemented!()
    }

    fn ids(&self) -> Vec<GeometryId> {
        unimplemented!()
    }
}

pub struct VulkanRenderer {
    textures: Rc<RefCell<VkTextureManager>>,
    geometries: Rc<RefCell<VkGeometryManager>>,
}

impl VulkanRenderer {
    pub fn new() -> Self {
        let textures = Rc::new(RefCell::new(VkTextureManager::new()));
        let geometries = Rc::new(RefCell::new(VkGeometryManager::new()));

        Self {
            textures,
            geometries,
        }
    }
}

impl RenderBackend for VulkanRenderer {
    fn get_geometry_manager(&self) -> SharedGeometryManager {
        self.geometries.clone()
    }

    fn get_texture_manager(&self) -> SharedTextureManager {
        self.textures.clone()
    }
}
