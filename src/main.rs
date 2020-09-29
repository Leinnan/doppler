#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate doppler;

mod example_client;

use doppler::engine::Engine;
use crate::example_client::ExampleClient;

pub fn main() {
    
    let engine = Engine::default();

    engine.run::<ExampleClient>();
}
