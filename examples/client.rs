use doppler::assets_cache::AssetsCache;
use doppler::camera::*;
use doppler::client::Client;
use doppler::glutin::event::{ElementState, VirtualKeyCode};
use doppler::map::*;

pub struct ExampleClient {
    map: Map,
    delta: f32,
}

impl Default for ExampleClient {
    fn default() -> Self {
        ExampleClient {
            delta: 0.0,
            map: Map::default(),
        }
    }
}

impl Client for ExampleClient {
    fn on_keyboard(&mut self, code: &VirtualKeyCode, state: &ElementState) {
        match (code, state) {
            (VirtualKeyCode::W, ElementState::Pressed) => self
                .map
                .camera
                .process_keyboard(CameraMovement::FORWARD, self.delta),
            (VirtualKeyCode::S, ElementState::Pressed) => self
                .map
                .camera
                .process_keyboard(CameraMovement::BACKWARD, self.delta),
            (VirtualKeyCode::A, ElementState::Pressed) => self
                .map
                .camera
                .process_keyboard(CameraMovement::LEFT, self.delta),
            (VirtualKeyCode::D, ElementState::Pressed) => self
                .map
                .camera
                .process_keyboard(CameraMovement::RIGHT, self.delta),
            (VirtualKeyCode::LControl, _) => self
                .map
                .camera
                .enable_mouse_movement(state == &ElementState::Released),
            (_, _) => (),
        }
    }

    fn load_assets(&mut self, cache: &mut AssetsCache) {
        cache.load_all_from_file("resources/test_objects.txt");
        self.map = MapSave::load("resources/test_map.yaml", cache);
    }

    unsafe fn draw(&mut self) {
        self.map.draw();
    }
    fn update(&mut self, delta: f32) {
        self.delta = delta;
    }

    fn on_mouse_scroll(&mut self, yoffset: f32) {
        self.map.camera.process_mouse_scroll(yoffset);
    }
    fn on_mouse_move(&mut self, x: f32, y: f32) {
        self.map.camera.process_mouse_movement(x, y, true);
    }
}

pub fn main() {
    doppler::engine::Engine::default().run::<ExampleClient>();
}
