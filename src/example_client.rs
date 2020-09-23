
use crate::gaia::camera::*;
use crate::gaia::consts;
use crate::gaia::*;
#[macro_use]
use imgui_glfw_rs::glfw;
use crate::gaia::macros;
use crate::gaia::engine::Engine;
use crate::gaia::client::Client;
use cgmath::{perspective, vec3, Deg, Matrix4, Point3, Rad, Vector3};
macro_rules! c_str {
    ($literal:expr) => {
        std::ffi::CStr::from_bytes_with_nul_unchecked(concat!($literal, "\0").as_bytes())
    };
}

pub struct ExampleClient {
    model: model::Model,
    camera: Camera,
    shader: shader::Shader,
}

impl ExampleClient {
    pub fn create() -> ExampleClient {
        ExampleClient {
            model: model::Model::new("resources/objects/nanosuit/nanosuit.obj"),
            camera: Camera {
                position: Point3::new(0.0, 0.0, 3.0),
                ..Camera::default()
            },
            shader: shader::Shader::from_file(
                "resources/shaders/model_loading.vs",
                "resources/shaders/model_loading.fs",
            ),
        }
    }
}

impl Client for ExampleClient {
    unsafe fn draw(&mut self) {
        self.shader.useProgram();

        // view/projection transformations
        let projection: Matrix4<f32> = perspective(
            Deg(self.camera.Zoom),
            consts::SCR_WIDTH as f32 / consts::SCR_HEIGHT as f32,
            0.1,
            100.0,
        );
        let view = self.camera.get_view_matrix();
        self.shader.setMat4(c_str!("projection"), &projection);
        self.shader.setMat4(c_str!("view"), &view);

        let mut m = Matrix4::<f32>::from_translation(vec3(3.0, -1.75, -1.25)); // translate it down so it's at the center of the scene
        m = m * Matrix4::from_scale(0.2); // it's a bit too big for our scene, so scale it down
        self.shader.setMat4(c_str!("model"), &m);
        self.model.Draw(&self.shader);
    }
    fn update(&mut self, engine: &mut Engine) {

    }

    
    fn process_input(&mut self, window: &glfw::Window, delta: f32) {
        use imgui_glfw_rs::glfw::{Action, Key};
        if window.get_key(Key::W) == Action::Press {
            self.camera
                .process_keyboard(Camera_Movement::FORWARD, delta);
        }
        if window.get_key(Key::S) == Action::Press {
            self.camera
                .process_keyboard(Camera_Movement::BACKWARD, delta);
        }
        if window.get_key(Key::A) == Action::Press {
            self.camera
                .process_keyboard(Camera_Movement::LEFT, delta);
        }
        if window.get_key(Key::D) == Action::Press {
            self.camera
                .process_keyboard(Camera_Movement::RIGHT, delta);
        }
        self.camera
            .enable_mouse_movement(window.get_key(Key::LeftControl) != Action::Press);
    }

    fn on_mouse_scroll(&mut self, yoffset: f32){
        self.camera.process_mouse_scroll(yoffset as f32);
    }
    fn on_mouse_move(&mut self, x: f32, y: f32){
        self.camera.process_mouse_movement(x, y, true);
    }
}