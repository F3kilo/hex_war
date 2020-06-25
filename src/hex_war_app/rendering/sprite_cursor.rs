use crate::graphics::resources::geometry::Geometry;
use crate::graphics::resources::texture::Texture;
use crate::graphics::Renderer;
use crate::hex_war_app::cursor::{CursorRenderer, State};
use crate::math::screen_coords::ScreenCoords;
use glam::Mat4;

pub struct Materials {
    pressed: Texture,
    released: Texture,
}

pub struct SpriteCursor {
    materials: Materials,
    current_material: Texture,
    geometry: Geometry,
    size: ScreenCoords,
}

impl SpriteCursor {
    pub fn new(materials: Materials, geometry: Geometry, size: ScreenCoords) -> Self {
        let current_material = materials.released.clone();
        Self {
            materials,
            current_material,
            geometry,
            size,
        }
    }
}

impl CursorRenderer for SpriteCursor {
    fn set_state(&mut self, state: State) {
        self.current_material = match state {
            State::Pressed => self.materials.pressed.clone(),
            State::Released => self.materials.released.clone(),
        };
    }

    fn render(&self, position: ScreenCoords) {
        // let scale = self.view.to_world(self.size);
        // let transforms = Mat4::from_scale_rotation_translation(scale);
    }
}
