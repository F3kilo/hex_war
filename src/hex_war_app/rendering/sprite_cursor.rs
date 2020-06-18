use crate::graphics::resources::material::SharedMaterial;
use crate::graphics::resources::view::View;
use crate::hex_war_app::cursor::{CursorRenderer, State};
use crate::math::screen_coords::ScreenCoords;
use crate::graphics::primitive::sprite2d::Sprite2d;

struct Materials {
    pressed: SharedMaterial,
    released: SharedMaterial,
}

pub struct SpriteCursor {
    sprite: Sprite2d,
    materials: Materials,
}

impl SpriteCursor {
    pub fn new(materials: Materials, view: View, sprite_builder: Sprite2dBuilder) -> Self {
        let sprite = sprite_builder.build(view, materials.released.clone());
        Self { sprite, materials }
    }
}

impl CursorRenderer for SpriteCursor {
    fn set_state(&mut self, state: State) {
        self.sprite.reset_material(match state {
            State::Pressed => self.materials.pressed.clone(),
            State::Released => self.materials.released.clone(),
        });
    }

    fn render(&self, position: ScreenCoords) {
        let world_position = self.sprite.get_view().from_screen(position);
        self.sprite.set_position(world_position);
        self.sprite.render();
    }
}
