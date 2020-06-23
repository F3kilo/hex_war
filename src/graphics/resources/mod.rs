use crate::graphics::resources::geometry::Geometry;
use crate::graphics::resources::material::Material;
use crate::graphics::resources::view::View;

pub mod geometry;
pub mod material;
pub mod texture;
pub mod view;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Resources {
    pub material: Material,
    pub geometry: Geometry,
    pub view: View,
}
