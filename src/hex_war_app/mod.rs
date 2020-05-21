pub mod error;
pub mod event;

use crate::app::status::Status;
use crate::app::{App, WinitEventAdaptor};
use error::{AppInitError, WindowCreateError};
use winit::event_loop::EventLoopWindowTarget;
use winit::window::{Window, WindowBuilder, WindowId};

pub struct HexWarApp {
    window: Window,
}

impl HexWarApp {
    pub fn new(event_loop_wt: &EventLoopWindowTarget<()>) -> Result<Self, AppInitError> {
        let window = Self::create_window(event_loop_wt)?;
        Ok(Self { window })
    }

    pub fn get_events_adaptor() -> impl WinitEventAdaptor<AppEvent = event::Event> {
        event::WinitEventAdaptor::new()
    }

    pub fn create_window(
        event_loop_wt: &EventLoopWindowTarget<()>,
    ) -> Result<Window, WindowCreateError> {
        WindowBuilder::new()
            .with_inner_size(winit::dpi::PhysicalSize::new(800, 600))
            .with_title("HexWar")
            .build(event_loop_wt)
    }
}

impl App for HexWarApp {
    type Event = event::Event;

    fn process_event(&mut self, event: Self::Event, wt: &EventLoopWindowTarget<()>) -> Status {
        if let event::Event::CloseRequested = event {
            return Status::Finish;
        }

        Status::Run
    }

    fn update(&mut self, _wt: &EventLoopWindowTarget<()>) -> Status {
        Status::Run
    }

    fn draw(&mut self, window_id: WindowId) {}
}
