use crate::coords::WorldCoords;

use crate::graphics::{Mesh, MeshInfo, Renderer, Texture};
use glam::{Mat4, Quat, Vec2, Vec4};

/// Four-component color. Last element represent transparency.
pub type ColorRgba = Vec4;

type UvCoords = Vec2;

pub struct Sprite2D {
    pos: WorldCoords,
    rotation: f32,
    size: WorldCoords,
    color: ColorRgba,
    texture: Texture,
    mesh: Mesh,
}

impl Sprite2D {
    /// Draws sprite on screen.
    pub fn draw(&self, renderer: &mut impl Renderer) {
        let rotation = Quat::from_rotation_z(self.rotation);
        let transforms = Mat4::from_scale_rotation_translation(
            self.size.get_inner(),
            rotation,
            self.pos.get_inner(),
        );

        let mesh_info = self.mesh_info();
        renderer.render_textured_mesh(mesh_info, self.texture)
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
