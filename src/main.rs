mod app;
mod event_loop;
mod hex_war_app;

#[macro_use]
extern crate slog;
use slog::{Drain, Logger};
use slog_async::Async;
use slog_term::{CompactFormat, TermDecorator};

use crate::hex_war_app::HexWarApp;
use std::error::Error;

fn main() {
    let logger = init_logger();
    info!(logger, "Logger initialized");

    let event_loop = winit::event_loop::EventLoop::new();
    let app = HexWarApp::new(&event_loop, logger.clone());
    if let Err(e) = app {
        show_error_message(e, logger);
        return;
    }
    let app = app.unwrap();

    event_loop::run_event_loop(event_loop, app, HexWarApp::get_events_adaptor());
}

fn init_logger() -> Logger {
    let term = TermDecorator::new().build();
    let format = CompactFormat::new(term).build().fuse();
    let sync = Async::new(format).build().fuse();
    Logger::root(sync, o!())
}

pub fn show_error_message(error: impl Error, logger: Logger) {
    let error_message = format!("Error occured: {}", error);
    tinyfiledialogs::message_box_ok(
        "Error",
        error_message.as_str(),
        tinyfiledialogs::MessageBoxIcon::Error,
    );
    crit!(logger, "Error occured: {}", error)
}
