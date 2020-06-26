use crate::graphics::resources::geometry::Geometry;
use crate::graphics::resources::scene::{Instance, Scene, TexturedGeometry, UvTransforms};
use crate::graphics::resources::texture::Texture;
use crate::graphics::Camera;
use crate::hex_war_app::cursor::{CursorRepresentation, State};
use crate::math::screen_coords::ScreenCoords;
use crate::math::world_coords::WorldCoords;
use glam::{Mat4, Quat};
use palette::Srgba;

pub struct Textures {
    pressed: Texture,
    released: Texture,
}

pub struct SpriteCursor {
    textures: Textures,
    current_texture: Texture,
    geometry: Geometry,
    size: ScreenCoords,
}

impl SpriteCursor {
    pub fn new(materials: Textures, geometry: Geometry, size: ScreenCoords) -> Self {
        let current_material = materials.released.clone();
        Self {
            textures: materials,
            current_texture: current_material,
            geometry,
            size,
        }
    }

    fn get_transforms(
        &self,
        camera: &impl Camera,
        position: ScreenCoords,
        screen_size: ScreenCoords,
    ) -> Mat4 {
        let depth = 0f32;
        let scale = self.calc_scale(screen_size, camera);
        let rotation = Quat::default();
        let translation = camera.to_world(position, depth, screen_size).get_inner();
        Mat4::from_scale_rotation_translation(scale.get_inner(), rotation, translation)
    }

    pub fn get_instance_data(
        &self,
        camera: &impl Camera,
        position: ScreenCoords,
        screen_size: ScreenCoords,
    ) -> Instance {
        Instance {
            transforms: self.get_transforms(camera, position, screen_size),
            uv_transforms: UvTransforms::default(),
            color: Srgba::new(1.0, 1.0, 1.0, 1.0),
        }
    }

    fn calc_scale(&self, screen_size: ScreenCoords, camera: &impl Camera) -> WorldCoords {
        let origin = camera.to_world(ScreenCoords::zero(), 0.0, screen_size);
        let point = camera.to_world(screen_size, 0.0, screen_size);
        point - origin
    }
}

impl CursorRepresentation for SpriteCursor {
    fn set_state(&mut self, state: State) {
        self.current_texture = match state {
            State::Pressed => self.textures.pressed.clone(),
            State::Released => self.textures.released.clone(),
        };
    }

    fn add_to_scene(
        &self,
        position: ScreenCoords,
        screen_size: ScreenCoords,
        scene: &mut Scene,
        camera: &impl Camera,
    ) {
        let tex_geom = TexturedGeometry {
            geometry: self.geometry.clone(),
            texture: self.current_texture.clone(),
            instance: self.get_instance_data(camera, position, screen_size),
        };
        scene.add_textured_geometry(tex_geom)
    }
}
