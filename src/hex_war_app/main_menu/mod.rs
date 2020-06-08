mod page;

use crate::hex_war_app::cursor::Cursor;
use crate::hex_war_app::event::CursorEvent;
use crate::hex_war_app::main_menu::page::PageEvent;
use page::Page;
use slog::Logger;

#[derive(Debug)]
pub enum MenuEvent {
    StartGame,
    Exit,
}

#[derive(Debug)]
pub struct MainMenu {
    page: Page,
    logger: Logger,
}

impl MainMenu {
    pub fn new(logger: Logger) -> Self {
        Self {
            page: Page::new(logger.clone()),
            logger,
        }
    }

    pub fn cursor_used(&mut self, event: CursorEvent, cursor: &mut Cursor) -> Option<MenuEvent> {
        trace!(self.logger, "MainMenu cursor event: {:?}", event);
        match event {
            CursorEvent::MoveTo(pos) => {
                self.page.cursor_moved(cursor);
                None
            }
            CursorEvent::ButtonUse(button, state) => {
                let page_event = self.page.button_used(button, state, cursor);
                self.process_page_event(page_event)
            }
        }
    }

    fn process_page_event(&mut self, event: Option<PageEvent>) -> Option<MenuEvent> {
        trace!(self.logger, "MainMenu page event: {:?}", event);

        if let Some(page_envent) = event {
            return match page_envent {
                PageEvent::PageChange(new_page) => {
                    self.change_page(new_page);
                    None
                }
                PageEvent::StartGame => Some(MenuEvent::StartGame),
                PageEvent::Exit => Some(MenuEvent::Exit),
            };
        }
        None
    }

    fn change_page(&mut self, new_page: Page) {
        trace!(self.logger, "MainMenu page change: {:?}", new_page);

        self.page = new_page;
    }

    pub fn render(&self) {
        self.page.render();
    }
}
