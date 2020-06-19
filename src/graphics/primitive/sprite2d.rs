use crate::graphics::resources::Resources;
use crate::graphics::{Renderer, Transforms};
use crate::math::world_coords::WorldCoords;
use glam::{Mat4, Quat};

pub struct Sprite2d {
    pos: WorldCoords,
    rotation: f32,
    size: WorldCoords,
    resources: Resources,
}

impl Sprite2d {
    pub fn new(resources: Resources, pos: WorldCoords, size: WorldCoords, rotation: f32) -> Self {
        Self {
            pos,
            rotation,
            size,
            resources,
        }
    }

    /// Draws sprite on screen.
    pub fn draw(&self, renderer: &Renderer) {
        let rotation = Quat::from_rotation_z(self.rotation);
        let instance_transforms = Mat4::from_scale_rotation_translation(
            self.size.get_inner(),
            rotation,
            self.pos.get_inner(),
        );
        let transforms = Transforms {
            instance: instance_transforms,
            uv: Default::default(),
        };
        renderer.render(&self.resources, transforms)
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

    pub fn get_resources(&self) -> &Resources {
        &self.resources
    }

    pub fn get_mut_resources(&mut self) -> &mut Resources {
        &mut self.resources
    }
}
