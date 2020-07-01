use crate::graphics::error::LoadError;
use crate::graphics::low_level::ProvideGeometryManager;
use crate::graphics::manager::geometry_manager::GeometryId;
use std::path::PathBuf;

pub struct Geometry<M: ProvideGeometryManager> {
    id: GeometryId,
    manager_provider: M,
}

impl<M: ProvideGeometryManager> Geometry<M> {
    pub fn new(path: PathBuf, mut manager_provider: M) -> Result<Self, LoadError> {
        let id = manager_provider
            .get_mut_geometry_manager()
            .create_geometry(path)?;
        Ok(Self {
            id,
            manager_provider,
        })
    }

    pub fn get_path(&self) -> PathBuf {
        self.manager_provider
            .get_geometry_manager()
            .get_path(self.id)
            .expect("Geometry id is not found, unexpectedly.")
    }
}

impl<M: ProvideGeometryManager> Drop for Geometry<M> {
    fn drop(&mut self) {
        self.manager_provider
            .get_mut_geometry_manager()
            .drop_geometry(self.id);
    }
}
