pub mod status;

use crate::app::status::Status;
use std::marker::PhantomData;
use winit::event::Event;
use winit::event_loop::EventLoopWindowTarget;
use winit::window::WindowId;

pub trait App {
    type Event;

    fn process_event(&mut self, event: Self::Event, wt: &EventLoopWindowTarget<()>) -> Status;
    fn update(&mut self, wt: &EventLoopWindowTarget<()>) -> Status;
    fn draw(&mut self, window_id: WindowId);
}

pub trait WinitEventAdaptor {
    type AppEvent;

    fn adapt_event<'a>(&'a self, event: Event<'a, ()>) -> Result<Self::AppEvent, Event<'a, ()>>;
}

#[derive(Default)]
pub struct DummyWinitEventAdaptor<AppEvent> {
    a: PhantomData<AppEvent>,
}

impl<T> WinitEventAdaptor for DummyWinitEventAdaptor<T> {
    type AppEvent = T;

    fn adapt_event<'a>(&'a self, event: Event<'a, ()>) -> Result<Self::AppEvent, Event<'a, ()>> {
        Err(event)
    }
}
