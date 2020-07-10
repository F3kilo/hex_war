use crate::graphics::error::{LoadError, NotFoundError};
use crate::graphics::manager::manage_geometries::{GeometryId, ManageGeometries};
use slog::Logger;
use std::path::PathBuf;

#[derive(Debug)]
pub struct VkGeometryManager {
    logger: Logger,
}

impl VkGeometryManager {
    pub fn new(logger: Logger) -> Self {
        Self { logger }
    }
}

impl ManageGeometries for VkGeometryManager {
    fn create_geometry(&mut self, path: PathBuf) -> Result<GeometryId, LoadError> {
        todo!()
    }

    fn drop_geometry(&mut self, id: GeometryId) -> bool {
        todo!()
    }

    fn get_path(&self, id: GeometryId) -> Result<PathBuf, NotFoundError> {
        todo!()
    }

    fn contains(&self, id: GeometryId) -> bool {
        todo!()
    }

    fn ids(&self) -> Vec<GeometryId> {
        todo!()
    }
}
