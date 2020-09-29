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
use log::info;
use log::LevelFilter;

pub fn main() {
    let _ = simple_logging::log_to_file("log.log", LevelFilter::Info);
    info!("Starting engine!");
    let engine = Engine::default();

    engine.run();
}
