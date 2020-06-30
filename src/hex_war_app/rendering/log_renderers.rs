use crate::hex_war_app::cursor::State;
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
