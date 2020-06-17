mod main_page;
use crate::hex_war_app::main_menu::page::main_page::MainPageEvent;
use crate::hex_war_app::LogCursor;
use main_page::MainPage;
use slog::Logger;
use winit::event::{ElementState, MouseButton};

#[derive(Debug)]
pub enum PageEvent {
    PageChange(Page),
    StartGame,
    Exit,
}

impl From<MainPageEvent> for PageEvent {
    fn from(event: MainPageEvent) -> Self {
        match event {
            MainPageEvent::StartGame => PageEvent::StartGame,
            MainPageEvent::Settings => PageEvent::PageChange(Page::Settings),
            MainPageEvent::Exit => PageEvent::Exit,
        }
    }
}

#[derive(Debug)]
pub enum Page {
    Main(MainPage),
    Settings,
}

impl Page {
    pub fn new(logger: Logger) -> Self {
        Page::Main(MainPage::new(logger))
    }

    pub fn cursor_moved(&mut self, cursor: &mut LogCursor) {
        match self {
            Page::Main(page) => page.cursor_moved(cursor),
            Page::Settings => {}
        }
    }

    pub fn button_used(
        &mut self,
        button: MouseButton,
        state: ElementState,
        cursor: &mut LogCursor,
    ) -> Option<PageEvent> {
        match self {
            Page::Main(page) => page.button_used(button, state, cursor).map(|e| e.into()),
            Page::Settings => None,
        }
    }

    pub fn render(&self) {
        match self {
            Page::Main(page) => page.render(),
            Page::Settings => {}
        }
    }
}
