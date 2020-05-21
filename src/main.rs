mod app;
mod event_loop;
mod hex_war_app;

#[macro_use]
extern crate slog;
use slog::{Drain, Logger};
use slog_async::Async;
use slog_term::{CompactFormat, TermDecorator};

use crate::app::DummyWinitEventAdaptor;
use crate::hex_war_app::HexWarApp;

fn main() {
    let logger = init_logger();
    info!(logger, "Logger initialized");

    let event_adapter = DummyWinitEventAdaptor::default();
    let app = HexWarApp {};

    event_loop::run_event_loop(app, event_adapter);
}

fn init_logger() -> Logger {
    let term = TermDecorator::new().build();
    let format = CompactFormat::new(term).build().fuse();
    let sync = Async::new(format).build().fuse();
    Logger::root(sync, o!())
}
