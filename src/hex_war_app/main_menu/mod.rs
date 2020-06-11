mod page;
use page::Page;
use slog::Logger;

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
}
