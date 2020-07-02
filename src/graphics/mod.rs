pub mod camera;
pub mod error;
pub mod geometry;
pub mod scene;
pub mod texture;

use crate::graphics::low_level::{GraphicsBackend, PresentInfo, RenderContext, RenderData};
use crate::graphics::manager::texture_manager::TextureId;
use crate::graphics::proxy::geometry_manager::GeometryManager;
use crate::graphics::proxy::texture_manager::TextureManager;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;

pub mod backend;
pub mod low_level;
pub mod manager;
pub mod primitive;
pub mod proxy;

#[derive(Clone)]
pub struct Graphics {
    backend: Rc<RefCell<Box<dyn GraphicsBackend>>>,
    texture_manager: TextureManager,
    geometry_manager: GeometryManager,
}

impl Graphics {
    pub fn new(backend: Box<dyn GraphicsBackend>) -> Self {
        let backend = Rc::new(RefCell::new(backend));
        let texture_manager = TextureManager::new(backend.clone());
        let geometry_manager = GeometryManager::new(backend.clone());
        Self {
            backend,
            texture_manager,
            geometry_manager,
        }
    }

    pub fn replace_backend(
        &mut self,
        new_backend: Box<dyn GraphicsBackend>,
    ) -> Box<dyn GraphicsBackend> {
        self.backend.borrow_mut().replace(new_backend)
    }

    pub fn get_geometry_manager(&self) -> &GeometryManager {
        &self.geometry_manager
    }

    pub fn get_mut_geometry_manager(&mut self) -> &mut GeometryManager {
        &mut self.geometry_manager
    }

    pub fn get_texture_manager(&self) -> &TextureManager {
        &self.texture_manager
    }

    pub fn get_mut_texture_manager(&mut self) -> &mut TextureManager {
        &mut self.texture_manager
    }

    pub fn render(&mut self, context: RenderContext, data: RenderData) -> TextureId {
        RefCell::borrow_mut(&self.backend).render(context, data)
    }

    pub fn present(&mut self, info: PresentInfo) {
        RefCell::borrow_mut(&self.backend).present(info)
    }
}
