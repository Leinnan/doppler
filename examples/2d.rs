use doppler::assets_cache::AssetsCache;
use doppler::camera::*;
use doppler::client::Client;
use doppler::glutin::event::{ElementState, VirtualKeyCode};
use doppler::shader::Shader;
use doppler::map::*;
use doppler::components::*;

pub struct Client2D{
    delta: f32,
    models_2d: ModelComponent,
    shader: Shader
}

impl Default for Client2D {
    fn default() -> Self {
        Client2D {
            delta: 0.0,
            models_2d: ModelComponent::default(),
            shader: Shader::from_file(
                "resources/shaders/multiple_lights.vs",
                "resources/shaders/multiple_lights.fs",
            )
        }
    }
}


impl Client for Client2D {
    fn on_keyboard(&mut self, code: &VirtualKeyCode, state: &ElementState) {
        match (code, state) {
            (VirtualKeyCode::W, ElementState::Pressed) => (),
            (VirtualKeyCode::S, ElementState::Pressed) => (),
            (VirtualKeyCode::A, ElementState::Pressed) => (),
            (VirtualKeyCode::D, ElementState::Pressed) => (),
            (VirtualKeyCode::LControl, _) => (),
            (_, _) => (),
        }
    }

    fn load_assets(&mut self, cache: &mut AssetsCache) {
        self.models_2d.model = cache.load_2d("resources/objects/ddd.jpg");
    }

    unsafe fn draw(&mut self) {
        use doppler::math::prelude::*;
        self.models_2d.draw(&self.shader);
        // let projection: Matrix4<f32> = perspective(
        //     Deg(self.camera.zoom),
        //     consts::SCR_WIDTH as f32 / consts::SCR_HEIGHT as f32,
        //     0.1,
        //     1000.0,
        // );
    }

    fn update(&mut self, delta: f32) {
        self.delta = delta;
    }

    fn debug_draw(&mut self, ui: &doppler::imgui::Ui) {
    }

    fn on_mouse_scroll(&mut self, yoffset: f32) {
    }

    fn on_mouse_move(&mut self, x: f32, y: f32) {
    }
}

pub fn main() {
    doppler::engine::Engine::default().run::<Client2D>();
}
