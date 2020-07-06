use crate::graphics::error::{LoadError, NotFoundError};
use crate::graphics::manager::manage_geometries::{GeometryId, ManageGeometries};
use std::path::PathBuf;

#[derive(Debug)]
pub struct VkGeometryManager {}

impl VkGeometryManager {
    pub fn new() -> Self {
        Self {}
    }
}

impl ManageGeometries for VkGeometryManager {
    fn create_geometry(&mut self, path: PathBuf) -> Result<GeometryId, LoadError> {
        unimplemented!()
    }

    fn drop_geometry(&mut self, id: GeometryId) -> bool {
        unimplemented!()
    }

    fn get_path(&self, id: GeometryId) -> Result<PathBuf, NotFoundError> {
        unimplemented!()
    }

    fn contains(&self, id: GeometryId) -> bool {
        unimplemented!()
    }

    fn ids(&self) -> Vec<GeometryId> {
        unimplemented!()
    }
}
