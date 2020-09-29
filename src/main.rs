#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate gl;
extern crate glutin;

#[cfg(feature = "imgui_inspect")]
extern crate imgui;
extern crate log;
extern crate simple_logging;

#[macro_use]
mod gaia;
mod example_client;

use crate::gaia::engine::Engine;
use log::LevelFilter;
use log::{info, trace, warn};

pub fn main() {
    simple_logging::log_to_file("log.log", LevelFilter::Info);
    info!("Starting engine!");
    let mut engine = Engine::default();

    engine.run();
}
