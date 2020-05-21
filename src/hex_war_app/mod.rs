use crate::app::status::Status;
use crate::app::App;
use winit::event_loop::EventLoopWindowTarget;
use winit::window::WindowId;

pub struct HexWarApp {}

impl App for HexWarApp {
    type Event = ();

    fn process_event(&mut self, event: Self::Event, wt: &EventLoopWindowTarget<()>) -> Status {
        unimplemented!()
    }

    fn update(&mut self, wt: &EventLoopWindowTarget<()>) -> Status {
        unimplemented!()
    }

    fn draw(&mut self, window_id: WindowId) {
        unimplemented!()
    }
}
