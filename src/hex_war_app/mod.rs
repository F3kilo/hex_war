pub mod event;

use crate::app::status::Status;
use crate::app::App;
use slog::Logger;
use winit::event_loop::EventLoopWindowTarget;
use winit::window::{Window, WindowId};

pub struct HexWarApp {
    window: Window,
    logger: Logger,
}

impl HexWarApp {
    pub fn new(window: Window, logger: Logger) -> Self {
        Self { window, logger }
    }
}

impl App for HexWarApp {
    type Event = event::Event;

    fn process_event(&mut self, event: Self::Event, _wt: &EventLoopWindowTarget<()>) -> Status {
        trace!(self.logger, "Called process_event({:?})", event);
        if let event::Event::CloseRequested = event {
            return Status::Finish;
        }

        Status::Run
    }

    fn update(&mut self, _wt: &EventLoopWindowTarget<()>) -> Status {
        trace!(self.logger, "Called update()");
        std::thread::sleep(std::time::Duration::from_millis(500));

        Status::Run
    }

    fn draw(&mut self, _window_id: WindowId) {
        trace!(self.logger, "Called draw()");
    }
}
