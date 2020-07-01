use crate::graphics::error::{LoadError, NotFoundError};
use std::path::PathBuf;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, Ord, PartialOrd)]
pub struct GeometryId(u64);

pub trait GeometryManager {
    fn create_geometry(&mut self, path: PathBuf) -> Result<GeometryId, LoadError>;
    fn drop_geometry(&mut self, id: GeometryId) -> bool;
    fn get_path(&self, id: GeometryId) -> Result<PathBuf, NotFoundError>;

    fn contains(&self, id: GeometryId) -> bool;
    fn ids(&self) -> Vec<GeometryId>;
}
