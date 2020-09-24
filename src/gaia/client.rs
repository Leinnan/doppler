use crate::gaia::engine::Engine;
use imgui_glfw_rs::glfw;

pub trait Client {
    fn update(&mut self, engine: &mut Engine);
    fn process_input(&mut self, window: &glfw::Window, delta: f32);
    fn on_mouse_scroll(&mut self, yoffset: f32);
    fn on_mouse_move(&mut self, x: f32, y: f32);
    // fn draw<T>(&mut self, engine: &mut Engine<T>) where T: Client;
    unsafe fn draw(&mut self);
    fn debug_draw(&mut self, ui: &imgui_glfw_rs::imgui::Ui);
}