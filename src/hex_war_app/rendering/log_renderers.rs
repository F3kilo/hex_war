use crate::graphics::resources::scene::Scene;
use crate::graphics::Camera;
use crate::hex_war_app::cursor::{CursorRepresentation, State};
use crate::math::screen_coords::ScreenCoords;
use slog::Logger;

pub struct CursorLogRenderer {
    logger: Logger,
    cursor_state: State,
}

impl CursorLogRenderer {
    pub fn new(logger: Logger, cursor_state: State) -> Self {
        Self {
            logger,
            cursor_state,
        }
    }
}

impl CursorRepresentation for CursorLogRenderer {
    fn set_state(&mut self, cursor_state: State) {
        self.cursor_state = cursor_state
    }

    fn add_to_scene(
        &self,
        position: ScreenCoords,
        _screen_size: ScreenCoords,
        _scene: &mut Scene,
        _camera: &impl Camera,
    ) {
        info!(
            self.logger,
            "Cursor: pos: ({:?}), state: {:?}", position, self.cursor_state
        )
    }
}
