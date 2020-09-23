#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate gl;
extern crate image;
extern crate imgui_glfw_rs;
use human_panic::setup_panic;
mod gaia;
use crate::gaia::engine::Engine;

pub fn main() {
    setup_panic!();
    let mut engine = Engine::default();

    engine.run();
}
