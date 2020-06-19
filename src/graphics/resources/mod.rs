use crate::graphics::resources::geometry::SharedGeometry;
use crate::graphics::resources::material::SharedMaterial;
use crate::graphics::resources::view::SharedView;

pub mod geometry;
pub mod material;
pub mod texture;
pub mod view;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Resources {
    pub material: SharedMaterial,
    pub geometry: SharedGeometry,
    pub view: SharedView,
}
