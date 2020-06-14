use crate::coords::WorldCoords;

use glam::{Vec4, Vec2};
use std::cell::Cell;

/// Four-component color. Last element represent transparency.
pub type ColorRgba = Vec4;

type UvCoords = Vec2;

struct TexturePart {
    pub texture: TextureHandle,
    pub uv: UvCoords,
    pub size: UvCoords,
}

type MutTexturePart = Cell<Box<dyn TexturePart>>;

pub struct Sprite2D {
    texture_part: TexturePart,
    pos: WorldCoords,
    rotation: f32,
    size: WorldCoords,
    color: ColorRgba,
}

impl Sprite2D {
    /// Draws sprite on screen.
    pub fn draw(&self, renderer: &impl RenderSprite) {
        let render_result = renderer.render(&self.texture_part, self.get_transforms(), self.color);
        if let Err(RenderError::TextureNotLoaded) = render_result {
            self.texture_part =
        }
    }

    /// Returns top left world position of sprite.
    pub fn get_position(&self) -> WorldCoords {
        self.pos
    }
    /// Moves spite top left corner world coordinates to  `pos`.
    pub fn move_to(&mut self, pos: WorldCoords) {
        self.pos = pos
    }
    /// Moves spite top left corner world coordinates by `shift`.
    pub fn move_by(&mut self, shift: WorldCoords) {
        self.pos += shift
    }

    /// Returns current rotation of sprite in range: ```[0, 2*Pi]```.
    pub fn get_rotation(&self) -> f32 {
        self.rotation
    }
    /// Rotates sprite to `angle`.
    pub fn rotate_to(&mut self, angle: f32) {
        self.rotation = angle
    }
    /// Rotates sprite by `angle`.
    pub fn rotate_by(&mut self, angle_shift: f32) {
        self.rotation += angle_shift
    }

    /// Returns current size of sprite in world coordinates.
    pub fn get_size(&self) -> WorldCoords {
        self.size
    }
    /// Sets size of sprite in world coordinates.
    pub fn set_size(&mut self, size: WorldCoords) {
        self.size = size
    }

    /// Returns color of sprite. Sprite texture color multiplies by it.
    pub fn get_color(&self) -> ColorRgba {
        self.color
    }
    /// Sets color of sprite. Sprite texture color multiplies by it.
    pub fn set_color(&mut self, color: ColorRgba) {
        self.color = color
    }
}
