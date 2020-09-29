use crate::gaia::assets_cache::AssetsCache;
use crate::gaia::engine::Engine;
use glutin::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};

pub trait Client {
    fn load_assets(&mut self, cache: &mut AssetsCache);
    fn update(&mut self, engine: &Engine, delta: f32);
    fn on_keyboard(&mut self, code: &VirtualKeyCode, state: &ElementState);
    fn on_mouse_scroll(&mut self, yoffset: f32);
    fn on_mouse_move(&mut self, x: f32, y: f32);
    unsafe fn draw(&mut self);
    #[cfg(feature = "imgui_inspect")]
    fn debug_draw(&mut self, ui: &imgui::Ui);
}
