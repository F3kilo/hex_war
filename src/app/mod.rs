use winit::event::Event;
use winit::event_loop::ControlFlow;
pub use winit::event_loop::EventLoopWindowTarget as ELWT;
use winit::window::WindowId;

pub trait App {
    type UserEvent;

    fn process_event(
        &mut self,
        event: Event<Self::UserEvent>,
        wt: &ELWT<Self::UserEvent>,
    ) -> ControlFlow;

    fn draw(&mut self, window_id: WindowId);
}
