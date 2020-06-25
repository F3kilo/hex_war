use crate::graphics::backend::RenderBackend;
use crate::graphics::resources::geometry::{Geometry, GeometryId, GeometryManager, UniqueGeometry};
use crate::graphics::resources::scene::{Scene, SceneId, SceneManager, TexturedGeometry};
use crate::graphics::resources::texture::{Texture, TextureId, TextureManager, UniqueTexture};
use crate::graphics::{LoadError, NotFoundError};
use crate::math::screen_coords::ScreenCoords;
use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;

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

    fn get_size(&self, id: TextureId) -> Result<ScreenCoords, NotFoundError> {
        unimplemented!()
    }

    fn get_path(&self, id: TextureId) -> Result<&Path, NotFoundError> {
        unimplemented!()
    }

    fn ids(&self) -> &dyn Iterator<Item = TextureId> {
        unimplemented!()
    }
}

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

    fn get_path(&self, id: GeometryId) -> Result<&Path, NotFoundError> {
        unimplemented!()
    }

    fn ids(&self) -> &dyn Iterator<Item = GeometryId> {
        unimplemented!()
    }
}

struct VkSceneManager {}

impl VkSceneManager {
    pub fn new() -> Self {
        Self {}
    }
}

impl SceneManager for VkSceneManager {
    fn create_scene(&mut self) -> SceneId {
        unimplemented!()
    }

    fn drop_scene(&mut self, id: SceneId) -> bool {
        unimplemented!()
    }

    fn add_textured_geometry(
        &mut self,
        id: SceneId,
        inst: TexturedGeometry,
    ) -> Result<(), NotFoundError> {
        unimplemented!()
    }

    fn ids(&self) -> &dyn Iterator<Item = SceneId> {
        unimplemented!()
    }
}

pub struct VulkanRenderer {
    textures: Rc<RefCell<VkTextureManager>>,
    geometries: Rc<RefCell<VkGeometryManager>>,
    scenes: Rc<RefCell<VkSceneManager>>,
}

impl VulkanRenderer {
    pub fn new() -> Self {
        let textures = Rc::new(RefCell::new(VkTextureManager::new()));
        let geometries = Rc::new(RefCell::new(VkGeometryManager::new()));
        let scenes = Rc::new(RefCell::new(VkSceneManager::new()));

        Self {
            textures,
            geometries,
            scenes,
        }
    }
}

impl RenderBackend for VulkanRenderer {
    fn create_texture(&self, path: PathBuf) -> Result<Texture, LoadError> {
        let unique_tex = UniqueTexture::new(path, self.textures.clone());
        unique_tex.map(|unique| Rc::new(unique))
    }

    fn create_geometry(&self, path: PathBuf) -> Result<Geometry, LoadError> {
        let unique_geom = UniqueGeometry::new(path, self.geometries.clone());
        unique_geom.map(|unique| Rc::new(unique))
    }

    fn create_scene(&self) -> Scene {
        Scene::new(self.scenes.clone())
    }
}
