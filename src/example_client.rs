
use crate::gaia::camera::*;
use crate::gaia::{consts,macros};
use crate::gaia::*;
use imgui_glfw_rs::glfw;
use crate::gaia::components::{Transform,ModelComponent};
use crate::gaia::engine::Engine;
use crate::gaia::client::Client;
use cgmath::{perspective, vec3, Deg, Matrix4, Point3, Rad, Vector3};

pub struct ExampleClient {
    model: ModelComponent,
    camera: Camera,
    shader: shader::Shader,
}

impl ExampleClient {
    pub fn create() -> ExampleClient {
        let mut t = Transform::default();
        t.position = vec3(3.0, -1.75, -1.25);
        t.scale = vec3(0.2,0.1,0.2);
        let model = ModelComponent {
            transform: t,
            model: model::Model::new("resources/objects/nanosuit/nanosuit.obj"),
        };
        ExampleClient {
            model: model,
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
        
        self.model.draw(&self.shader);
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