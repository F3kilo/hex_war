pub mod event;

use crate::app::status::Status;
use crate::app::App;
use crate::hex_war_app::event::{Event, WindowEvent};
use slog::Logger;
use winit::event_loop::EventLoopWindowTarget;
use winit::window::{Window, WindowId};

pub struct HexWarApp {
    window: Window,
    logger: Logger,
    status: Status,
}

impl HexWarApp {
    pub fn new(window: Window, logger: Logger) -> Self {
        Self {
            window,
            logger,
            status: Status::Run,
        }
    }

    pub fn get_status(&self) -> Status {
        self.status
    }

    fn process_window_event(&mut self, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => self.status = Status::Finish,
            WindowEvent::Cursor(_cursor_event) => {}
        }
    }
}

impl App for HexWarApp {
    type Event = event::Event;

    fn process_event(&mut self, event: Self::Event, _wt: &EventLoopWindowTarget<()>) {
        trace!(self.logger, "Called process_event({:?})", event);

        match event {
            Event::Window(window_event) => self.process_window_event(window_event),
        }
    }

    fn update(&mut self, _wt: &EventLoopWindowTarget<()>) -> Status {
        trace!(self.logger, "Called update()");
        std::thread::sleep(std::time::Duration::from_millis(200));

        self.status
    }

    fn draw(&mut self, _window_id: WindowId) {
        trace!(self.logger, "Called draw()");
    }
}
