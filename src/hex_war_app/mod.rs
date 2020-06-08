pub mod event;
pub mod main_menu;
pub mod state;

use crate::app::status::Status;
use crate::app::App;
use crate::hex_war_app::state::StateEvent;
use event::{Event, WindowEvent};
use main_menu::MainMenu;
use slog::Logger;
use state::State;
use winit::event_loop::EventLoopWindowTarget;
use winit::window::{Window, WindowId};

pub struct HexWarApp {
    window: Window,
    logger: Logger,
    status: Status,
    state: State,
}

impl HexWarApp {
    pub fn new(window: Window, logger: Logger) -> Self {
        Self {
            window,
            logger: logger.clone(),
            status: Status::Run,
            state: State::Menu(MainMenu::new(logger)),
        }
    }

    pub fn get_status(&self) -> Status {
        self.status
    }

    fn process_window_event(&mut self, event: WindowEvent) {
        trace!(
            self.logger,
            "HexWarApp processing window event: {:?}",
            event
        );
        match event {
            WindowEvent::CloseRequested => self.status = Status::Finish,
            WindowEvent::Cursor(cursor_event) => {
                let state_event = self.state.cursor_used(cursor_event);
                self.process_state_event(state_event);
            }
        }
    }

    fn process_state_event(&mut self, event: Option<StateEvent>) {
        trace!(self.logger, "HexWarApp processing state event: {:?}", event);

        if let Some(event) = event {
            match event {
                StateEvent::Exit => self.status = Status::Finish,
                StateEvent::StartGame => self.start_game(),
            }
        }
    }

    fn start_game(&mut self) {
        trace!(self.logger, "HexWarApp start game");

        self.state = State::Game;
    }

    fn render(&mut self) {
        trace!(self.logger, "HexWarApp render called");

        self.state.render();
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

        self.get_status()
    }

    fn draw(&mut self, _window_id: WindowId) {
        trace!(self.logger, "Called draw()");
        self.render()
    }
}
