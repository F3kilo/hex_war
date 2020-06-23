use crate::graphics::Renderer;
use crate::hex_war_app::cursor::{CursorRenderer, State};
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

impl CursorRenderer for CursorLogRenderer {
    fn set_state(&mut self, cursor_state: State) {
        self.cursor_state = cursor_state
    }

    fn render(&self, position: ScreenCoords) {
        info!(
            self.logger,
            "Cursor: pos: ({:?}), state: {:?}", position, self.cursor_state
        )
    }
}
