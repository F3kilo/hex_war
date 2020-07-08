use crate::graphics::geometry::Geometry;
use crate::graphics::manager::manage_scenes::{Instance, TexturedGeometry};
use crate::graphics::texture::Texture;
use crate::math::world_coords::WorldCoords;
use glam::{Mat4, Quat, Vec3};
use palette::Srgba;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum State {
    Pressed,
    Released,
}

#[derive(Debug)]
pub struct Resources {
    pub pressed: Texture,
    pub released: Texture,
    pub unit_quad: Geometry,
}

#[derive(Debug)]
pub struct SpriteCursor {
    resources: Resources,
    scale: f32,
}

impl SpriteCursor {
    pub fn new(resources: Resources, scale: f32) -> Self {
        Self { resources, scale }
    }

    // pub fn add_to_scene(&self, scene: &mut Scene, state: State, pos: WorldCoords) {
    //     let texture = match state {
    //         State::Pressed => self.resources.pressed.clone(),
    //         State::Released => self.resources.released.clone(),
    //     };
    //
    //     let instance = Instance {
    //         transforms: Mat4::from_scale_rotation_translation(
    //             Vec3::splat(self.scale),
    //             Quat::default(),
    //             pos.get_inner(),
    //         ),
    //         uv_transforms: Default::default(),
    //         color: Srgba::new(1.0, 1.0, 1.0, 1.0),
    //     };
    //
    //
    //     let textured_geometry = TexturedGeometry {
    //         geometry: self.resources.unit_quad.clone(),
    //         texture,
    //         instance,
    //     };
    //
    //     scene.add_textured_geometry(textured_geometry)
    // }
}

#[derive(Debug)]
pub struct Cursor {
    pos: WorldCoords,
    representation: SpriteCursor,
    state: State,
}

impl Cursor {
    pub fn new(pos: WorldCoords, representation: SpriteCursor) -> Self {
        Self {
            pos,
            representation,
            state: State::Released,
        }
    }

    pub fn get_pos(&self) -> WorldCoords {
        self.pos
    }

    pub fn move_to(&mut self, pos: WorldCoords) {
        self.pos = pos
    }

    pub fn change_state(&mut self, new_state: State) {
        self.state = new_state
    }

    // pub fn add_to_scene(&self, scene: &mut Scene) {
    //     self.representation qwe
    //         .add_to_scene(scene, self.state, self.pos)
    // }
}
