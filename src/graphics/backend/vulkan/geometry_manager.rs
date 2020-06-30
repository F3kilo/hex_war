use crate::graphics::manager::geometry_manager::{GeometryId, GeometryManager};
use crate::graphics::{LoadError, NotFoundError};
use std::path::PathBuf;

#[derive(Debug)]
pub struct VkGeometryManager {}

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
