pub mod camera;
pub mod error;
pub mod geometry;
pub mod scene;
pub mod texture;

use crate::graphics::backend::{GraphicsBackend, PresentInfo};
use crate::graphics::proxy::geometry_manager::GeometryManager;
use crate::graphics::proxy::scene_manager::SceneManager;
use crate::graphics::proxy::texture_manager::TextureManager;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;

pub mod backend;
pub mod manager;
pub mod primitive;
pub mod proxy;

#[derive(Clone)]
pub struct Graphics {
    backend: Rc<RefCell<Box<dyn GraphicsBackend>>>,
    texture_manager: TextureManager,
    geometry_manager: GeometryManager,
    scene_manager: SceneManager,
}

impl Graphics {
    pub fn new(backend: Box<dyn GraphicsBackend>) -> Self {
        let backend = Rc::new(RefCell::new(backend));
        let texture_manager = TextureManager::new(backend.clone());
        let geometry_manager = GeometryManager::new(backend.clone());
        let scene_manager = SceneManager::new(backend.clone());
        Self {
            backend,
            texture_manager,
            geometry_manager,
            scene_manager,
        }
    }

    pub fn replace_backend(
        &mut self,
        new_backend: Box<dyn GraphicsBackend>,
    ) -> Box<dyn GraphicsBackend> {
        self.backend.borrow_mut().replace(new_backend)
    }

    pub fn get_geometry_manager(&self) -> GeometryManager {
        self.geometry_manager.clone()
    }

    pub fn get_texture_manager(&self) -> TextureManager {
        self.texture_manager.clone()
    }

    pub fn get_scene_manager(&self) -> SceneManager {
        self.scene_manager.clone()
    }

    pub fn present(&mut self, info: PresentInfo) {
        RefCell::borrow_mut(&self.backend).present(info)
    }
}
