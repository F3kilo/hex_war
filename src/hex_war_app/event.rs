use glam::Vec2;
use winit::event::{ElementState, MouseButton};

#[derive(Debug)]
pub enum Event {
    Window(WindowEvent),
}

#[derive(Debug)]
pub enum WindowEvent {
    Cursor(CursorEvent),
    CloseRequested,
}

#[derive(Debug)]
pub enum CursorEvent {
    MoveTo(Vec2),
    ButtonUse(MouseButton, ElementState),
}
