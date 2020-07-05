#![allow(dead_code)]
#![allow(unused_variables)]

mod app;
mod app_init_error;
mod event_loop;
mod graphics;
mod hex_war_app;
mod math;

#[macro_use]
extern crate slog;
use slog::{Drain, Logger};
use slog_async::Async;
use slog_term::{CompactFormat, TermDecorator};

use crate::app_init_error::{AppInitError, WindowCreateError};
use crate::hex_war_app::HexWarApp;
use std::error::Error;
use winit::event_loop::{EventLoop, EventLoopWindowTarget};
use winit::window::{Window, WindowBuilder};

fn main() {
    let logger = init_logger();
    info!(logger, "Logger initialized");

    let event_loop = winit::event_loop::EventLoop::new();
    let app = init_hex_war_app(logger.clone(), &event_loop);
    if let Err(e) = app {
        show_error_message(e, logger);
        return;
    }
    let app = app.unwrap();

    event_loop::run_event_loop(event_loop, app);
}

fn init_logger() -> Logger {
    let term = TermDecorator::new().build();
    let format = CompactFormat::new(term).build().fuse();
    let sync = Async::new(format).build().fuse();
    Logger::root(sync, o!())
}

fn init_window(event_loop_wt: &EventLoopWindowTarget<()>) -> Result<Window, WindowCreateError> {
    WindowBuilder::new()
        .with_inner_size(winit::dpi::PhysicalSize::new(800, 600))
        .with_title("HexWar")
        .build(event_loop_wt)
}

fn init_hex_war_app(logger: Logger, elwt: &EventLoop<()>) -> Result<HexWarApp, AppInitError> {
    let window = init_window(&elwt)?;
    Ok(HexWarApp::new(window, logger))
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
