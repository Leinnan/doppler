use crate::gaia::engine::Engine;
use imgui_glfw_rs::glfw;
use crate::gaia::assets_cache::AssetsCache;

pub trait Client {
    fn load_assets(&mut self, cache: &mut AssetsCache);
    fn update(&mut self, engine: &mut Engine);
    fn process_input(&mut self, window: &glfw::Window, delta: f32);
    fn on_mouse_scroll(&mut self, yoffset: f32);
    fn on_mouse_move(&mut self, x: f32, y: f32);
    // fn draw<T>(&mut self, engine: &mut Engine<T>) where T: Client;
    unsafe fn draw(&mut self);
    fn debug_draw(&mut self, ui: &imgui_glfw_rs::imgui::Ui);
}
