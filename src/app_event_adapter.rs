use crate::hex_war_app::event::Event;

pub struct WinitEventAdaptor {}

impl WinitEventAdaptor {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::app::WinitEventAdaptor for WinitEventAdaptor {
    type AppEvent = Event;

    fn adapt_event<'a>(
        &'a self,
        event: winit::event::Event<'a, ()>,
    ) -> Result<Self::AppEvent, winit::event::Event<'a, ()>> {
        match event {
            winit::event::Event::WindowEvent {
                event: winit::event::WindowEvent::CloseRequested,
                ..
            } => Ok(Event::CloseRequested),
            _ => Err(event),
        }
    }
}
