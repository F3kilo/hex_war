pub mod vulkan;

use crate::graphics::manager::geometry_manager::SharedGeometryManager;
use crate::graphics::manager::texture_manager::SharedTextureManager;

pub trait RenderBackend {
    fn get_geometry_manager(&self) -> SharedGeometryManager;
    fn get_texture_manager(&self) -> SharedTextureManager;
}
