use crate::graphics::geometry::Geometry;
use crate::graphics::manager::manage_scenes::Instance;
use crate::graphics::scene::{Scene, TexturedGeometry};
use crate::graphics::texture::Texture;
use crate::hex_war_app::ResourceManagers;
use crate::math::world_coords::WorldCoords;
use glam::{Mat4, Quat, Vec3};
use palette::Srgba;
use std::path::PathBuf;

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
    repr: CursorRepr,
    state: State,
    scale: f32,
}

impl Cursor {
    pub fn new(managers: ResourceManagers) -> Self {
        Self {
            pos: WorldCoords::default(),
            repr: CursorRepr::new(managers),
            state: State::Released,
            scale: 1.0,
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

    pub fn add_to_scene(&self, scene: &mut Scene) {
        self.repr
            .add_to_scene(scene, self.pos, self.scale, self.state)
    }
}

#[derive(Debug)]
pub struct CursorRepr {
    pressed_tex: Texture,
    released_tex: Texture,
    geom: Geometry,
}

impl CursorRepr {
    pub fn new(managers: ResourceManagers) -> Self {
        let pressed_tex = Texture::new(Self::pressed_tex_path(), managers.tex_manager.clone())
            .expect("Can't load pressed cursor texture.");

        let released_tex = Texture::new(Self::released_tex_path(), managers.tex_manager)
            .expect("Can't load released cursor texture.");

        let geom = Geometry::new(Self::geom_path(), managers.geom_manager)
            .expect("Can't load quad geometry.");

        Self {
            pressed_tex,
            released_tex,
            geom,
        }
    }

    fn pressed_tex_path() -> PathBuf {
        "resources/textures/pressed_cursor.ktx".into()
    }

    fn released_tex_path() -> PathBuf {
        "resources/textures/pressed_cursor.ktx".into()
    }

    fn geom_path() -> PathBuf {
        "resources/geometries/quad.ktx".into()
    }

    pub fn add_to_scene(&self, scene: &mut Scene, pos: WorldCoords, scale: f32, state: State) {
        let texture = match state {
            State::Pressed => self.pressed_tex.clone(),
            State::Released => self.released_tex.clone(),
        };

        let instance = Instance {
            transforms: Mat4::from_scale_rotation_translation(
                Vec3::splat(scale),
                Quat::default(),
                pos.get_inner(),
            ),
            uv_transforms: Default::default(),
            color: Srgba::new(1.0, 1.0, 1.0, 1.0),
        };

        let textured_geometry = TexturedGeometry {
            geometry: self.geom.clone(),
            texture,
            instance,
        };

        scene.add_item(textured_geometry.into())
    }
}
