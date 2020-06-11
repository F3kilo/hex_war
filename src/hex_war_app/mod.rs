pub mod cursor;
pub mod main_menu;
pub mod state;

use crate::app::{App, ELWT};
use crate::hex_war_app::cursor::Cursor;
use crate::screen_coords::ScreenCoords;
use glam::Vec2;
use main_menu::MainMenu;
use slog::Logger;
use state::State;
use winit::event::{ElementState, Event, MouseButton, StartCause, WindowEvent};
use winit::event_loop::ControlFlow;
use winit::window::{Window, WindowId};

pub struct HexWarApp {
    window: Window,
    logger: Logger,
    state: State,
    cursor: Cursor,
}

impl HexWarApp {
    pub fn new(window: Window, logger: Logger) -> Self {
        let cursor = Cursor::new(ScreenCoords::zero());
        Self {
            window,
            logger: logger.clone(),
            state: State::Menu(MainMenu::new(logger)),
            cursor,
        }
    }

    pub fn close_requested(&mut self) {}

    pub fn cursor_moved(&mut self, new_pos: ScreenCoords) {
        self.cursor.move_to(new_pos)
    }

    pub fn mouse_button_used(&mut self, button: MouseButton, state: ElementState) {}

    pub fn is_time_to_update(&self) -> bool {
        true
    }

    pub fn update(&mut self) -> bool {
        true
    }

    pub fn is_finished(&self) -> bool {
        false
    }

    pub fn window_resized(&mut self, new_size: ScreenCoords) {}
    pub fn window_destroyed(&mut self) {}
    pub fn draw(&mut self) {}
}

impl App for HexWarApp {
    type UserEvent = ();

    fn process_event(
        &mut self,
        event: Event<Self::UserEvent>,
        _wt: &ELWT<Self::UserEvent>,
    ) -> ControlFlow {
        let mut control_flow = ControlFlow::Poll;
        match event {
            Event::NewEvents(cause) => {
                if let StartCause::Poll = cause {
                    if self.is_time_to_update() {
                        self.update();
                    }
                }
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(new_size) => {
                    let new_size: (u32, u32) = new_size.into();
                    self.window_resized(new_size.into());
                }
                WindowEvent::Moved(_) => {}
                WindowEvent::CloseRequested => self.close_requested(),
                WindowEvent::Destroyed => self.window_destroyed(),
                WindowEvent::DroppedFile(_) => {}
                WindowEvent::HoveredFile(_) => {}
                WindowEvent::HoveredFileCancelled => {}
                WindowEvent::ReceivedCharacter(_) => {}
                WindowEvent::Focused(_) => {}
                WindowEvent::KeyboardInput { .. } => {}
                WindowEvent::ModifiersChanged(_) => {}
                WindowEvent::CursorMoved { position, .. } => {
                    let new_pos = (position.x as u32, position.y as u32).into();
                    self.cursor_moved(new_pos)
                }
                WindowEvent::CursorEntered { .. } => {}
                WindowEvent::CursorLeft { .. } => {}
                WindowEvent::MouseWheel { .. } => {}
                WindowEvent::MouseInput { button, state, .. } => {
                    self.mouse_button_used(button, state)
                }
                WindowEvent::TouchpadPressure { .. } => {}
                WindowEvent::AxisMotion { .. } => {}
                WindowEvent::Touch(_) => {}
                WindowEvent::ScaleFactorChanged { .. } => {}
                WindowEvent::ThemeChanged(_) => {}
            },
            Event::DeviceEvent { .. } => {}
            Event::UserEvent(_) => {}
            Event::Suspended => {}
            Event::Resumed => {}
            Event::MainEventsCleared => {}
            Event::RedrawRequested(_) => self.draw(),
            Event::RedrawEventsCleared => {}
            Event::LoopDestroyed => {}
        };

        if self.is_finished() {
            control_flow = ControlFlow::Exit;
        }

        control_flow
    }

    fn draw(&mut self, window_id: WindowId) {
        unimplemented!()
    }
}
