#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
extern crate doppler;
mod example_client;
use crate::example_client::ExampleClient;

pub fn main() {
    doppler::engine::Engine::default().run::<ExampleClient>();
}
