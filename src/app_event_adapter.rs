use crate::hex_war_app::event::{CursorEvent, Event, WindowEvent};
use glam::Vec2;
use winit::event::{ElementState, MouseButton};

pub struct WinitEventAdaptor {}

impl WinitEventAdaptor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn process_window_event<'a>(
        &self,
        event: winit::event::WindowEvent<'a>,
        window_id: winit::window::WindowId,
    ) -> Result<Event, winit::event::Event<'a, ()>> {
        match event {
            winit::event::WindowEvent::CloseRequested => {
                Ok(Event::Window(WindowEvent::CloseRequested))
            }
            winit::event::WindowEvent::CursorMoved { position, .. } => {
                Ok(Self::move_cursor_event(position))
            }
            winit::event::WindowEvent::MouseInput { state, button, .. } => {
                Ok(Self::cursor_button_used(button, state))
            }
            _ => Err(winit::event::Event::WindowEvent { event, window_id }),
        }
    }

    fn move_cursor_event(position: winit::dpi::PhysicalPosition<f64>) -> Event {
        Event::Window(WindowEvent::Cursor(CursorEvent::MoveTo(Vec2::new(
            position.x as f32,
            position.y as f32,
        ))))
    }

    fn cursor_button_used(button: MouseButton, state: ElementState) -> Event {
        Event::Window(WindowEvent::Cursor(CursorEvent::ButtonUse(button, state)))
    }
}

impl crate::app::WinitEventAdaptor for WinitEventAdaptor {
    type AppEvent = Event;

    fn adapt_event<'a>(
        &'a self,
        event: winit::event::Event<'a, ()>,
    ) -> Result<Self::AppEvent, winit::event::Event<'a, ()>> {
        match event {
            winit::event::Event::WindowEvent { event, window_id } => {
                self.process_window_event(event, window_id)
            }
            _ => Err(event),
        }
    }
}
