use crate::graphics::error::{LoadError, UnavailableError};
use std::path::PathBuf;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, Ord, PartialOrd)]
pub struct GeometryId(u64);

pub trait ManageGeometries {
    fn create_geometry(&mut self, path: PathBuf) -> Result<GeometryId, LoadError>;
    fn drop_geometry(&mut self, id: GeometryId) -> bool;
    fn get_path(&self, id: GeometryId) -> Result<PathBuf, UnavailableError>;

    fn contains(&self, id: GeometryId) -> bool;
    fn ids(&self) -> Vec<GeometryId>;
}
