pub mod geometry;
pub mod camera;
pub mod error;
pub mod texture;

use crate::graphics::error::{LoadError, NotFoundError};
use crate::graphics::low_level::{
    Graphics, Present, PresentInfo, ProvideGeometryManager, ProvideRenderer, ProvideTextureManager,
    Render,
};
use crate::graphics::manager::geometry_manager::GeometryManager;
use crate::graphics::manager::texture_manager::TextureManager;
use crate::graphics::proxy::geometry_manager::GeometryManagerProxy;
use crate::graphics::proxy::texture_manager::TextureManagerProxy;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;

pub mod backend;
pub mod low_level;
pub mod manager;
pub mod primitive;
pub mod proxy;

#[derive(Clone)]
pub struct SharedGraphics {
    backend: Rc<RefCell<Box<dyn Graphics>>>,
    texture_manager: TextureManagerProxy,
    geometry_manager: GeometryManagerProxy,
}

impl SharedGraphics {
    pub fn new(backend: Box<dyn Graphics>) -> Self {
        let backend = Rc::new(RefCell::new(backend));
        let texture_manager = TextureManagerProxy::new(backend.clone());
        let geometry_manager = GeometryManagerProxy::new(backend.clone());
        Self {
            backend,
            texture_manager,
            geometry_manager,
        }
    }

    pub fn replace_backend(&mut self, new_backend: Box<dyn Graphics>) -> Box<dyn Graphics> {
        self.backend.borrow_mut().replace(new_backend)
    }
}

impl ProvideTextureManager for SharedGraphics {
    fn get_mut_texture_manager(&mut self) -> &mut dyn TextureManager {
        &mut self.texture_manager
    }

    fn get_texture_manager(&self) -> &dyn TextureManager {
        &self.texture_manager
    }
}

impl ProvideGeometryManager for SharedGraphics {
    fn get_mut_geometry_manager(&mut self) -> &mut dyn GeometryManager {
        &mut self.geometry_manager
    }

    fn get_geometry_manager(&self) -> &dyn GeometryManager {
        &self.geometry_manager
    }
}

impl ProvideRenderer for SharedGraphics {
    fn get_mut_renderer(&mut self) -> &mut dyn Render {
        unimplemented!()
    }

    fn get_renderer(&self) -> &dyn Render {
        unimplemented!()
    }
}

impl Present for SharedGraphics {
    fn present(&mut self, info: PresentInfo) {
        unimplemented!()
    }
}

impl Graphics for SharedGraphics {}
