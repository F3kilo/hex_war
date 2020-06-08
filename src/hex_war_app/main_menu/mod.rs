mod cursor;
mod page;

use crate::hex_war_app::event::CursorEvent;
use crate::hex_war_app::main_menu::page::PageEvent;
use cursor::Cursor;
use page::Page;

#[derive(Debug)]
pub enum MenuEvent {
    StartGame,
    Exit,
}

#[derive(Debug)]
pub struct MainMenu {
    page: Page,
    cursor: Cursor,
}

impl MainMenu {
    pub fn new() -> Self {
        Self {
            page: Page::new(),
            cursor: Cursor::new((0f32, 0f32).into()),
        }
    }

    pub fn cursor_used(&mut self, event: CursorEvent) -> Option<MenuEvent> {
        match event {
            CursorEvent::MoveTo(pos) => {
                self.cursor.move_to(pos);
                self.page.cursor_moved(&self.cursor);
                None
            }
            CursorEvent::ButtonUse(button, state) => {
                let page_event = self.page.button_used(button, state, &self.cursor);
                self.process_page_event(page_event)
            }
        }
    }

    fn process_page_event(&mut self, event: Option<PageEvent>) -> Option<MenuEvent> {
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
        self.page = new_page;
    }

    pub fn render(&self) {
        self.cursor.render();
        self.page.render();
    }
}
