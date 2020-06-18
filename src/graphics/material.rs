use crate::graphics::texture::SharedTexture;
use palette::Srgba;
use std::mem;

#[derive(Debug, Clone, PartialEq)]
pub struct Material {
    color_map: SharedTexture,
    color: Srgba,
}

impl Material {
    pub fn new(color_map: SharedTexture, color: Srgba) -> Self {
        Self { color_map, color }
    }

    pub fn get_color_map(&self) -> &SharedTexture {
        &self.color_map
    }

    pub fn reset_color_map(&mut self, mut new_color_map: SharedTexture) -> SharedTexture {
        mem::swap(&mut self.color_map, &mut new_color_map);
        new_color_map
    }

    pub fn get_color(&self) -> Srgba {
        self.color
    }

    pub fn set_color(&mut self, color: Srgba) {
        self.color = color
    }
}
