#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate gl;

#[cfg(feature = "imgui_inspect")]
extern crate imgui_glfw_rs;
extern crate log;
extern crate simple_logging;

#[macro_use]
mod gaia;
mod example_client;

use crate::gaia::engine::Engine;
// use human_panic::setup_panic;
use log::LevelFilter;
use log::{info, trace, warn};

pub fn main() {
    // setup_panic!();
    simple_logging::log_to_file("log.log", LevelFilter::Info);
    info!("Starting engine!");
    let mut engine = Engine::default();

    engine.run();
}
