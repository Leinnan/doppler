use crate::gaia::camera::*;
use crate::gaia::client::Client;
use crate::gaia::components::{ModelComponent, Transform};
use crate::gaia::consts;
use crate::gaia::engine::Engine;
use crate::gaia::*;
use cgmath::{perspective, vec3, Deg, Matrix4, Point3};
use imgui_glfw_rs::glfw;

pub struct ExampleClient {
    model: ModelComponent,
    camera: Camera,
    shader: shader::Shader,
    show_object_info: bool,
}

impl ExampleClient {
    pub fn create() -> ExampleClient {
        let mut t = Transform::default();
        t.position = vec3(3.0, -1.75, -1.25);
        t.scale = vec3(0.2, 0.1, 0.2);
        let model = ModelComponent {
            transform: t,
            model: model::Model::new("resources/objects/nanosuit/nanosuit.obj"),
        };
        ExampleClient {
            show_object_info: true,
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
        self.shader.use_program();

        // view/projection transformations
        let projection: Matrix4<f32> = perspective(
            Deg(self.camera.zoom),
            consts::SCR_WIDTH as f32 / consts::SCR_HEIGHT as f32,
            0.1,
            100.0,
        );
        let view = self.camera.get_view_matrix();
        self.shader.set_mat4(c_str!("projection"), &projection);
        self.shader.set_mat4(c_str!("view"), &view);

        self.model.draw(&self.shader);
    }
    fn update(&mut self, _engine: &mut Engine) {}

    fn process_input(&mut self, window: &glfw::Window, delta: f32) {
        use imgui_glfw_rs::glfw::{Action, Key};
        if window.get_key(Key::W) == Action::Press {
            self.camera.process_keyboard(CameraMovement::FORWARD, delta);
        }
        if window.get_key(Key::S) == Action::Press {
            self.camera
                .process_keyboard(CameraMovement::BACKWARD, delta);
        }
        if window.get_key(Key::A) == Action::Press {
            self.camera.process_keyboard(CameraMovement::LEFT, delta);
        }
        if window.get_key(Key::D) == Action::Press {
            self.camera.process_keyboard(CameraMovement::RIGHT, delta);
        }
        self.camera
            .enable_mouse_movement(window.get_key(Key::LeftControl) != Action::Press);
    }

    fn debug_draw(&mut self, ui: &imgui_glfw_rs::imgui::Ui) {
        use imgui_glfw_rs::imgui::*;
        use imgui_inspect::InspectArgsStruct;
        if let Some(menu_bar) = ui.begin_main_menu_bar() {
            if let Some(menu) = ui.begin_menu(im_str!("Basic"), true) {
                if MenuItem::new(im_str!("Show Object info"))
                    .selected(self.show_object_info)
                    .build(ui)
                {
                    self.show_object_info = !self.show_object_info;
                }
                menu.end(ui);
            }
            if let Some(menu) = ui.begin_menu(im_str!("Edit"), true) {
                MenuItem::new(im_str!("Undo"))
                    .shortcut(im_str!("CTRL+Z"))
                    .build(ui);
                MenuItem::new(im_str!("Redo"))
                    .shortcut(im_str!("CTRL+Y"))
                    .enabled(false)
                    .build(ui);
                ui.separator();
                MenuItem::new(im_str!("Cut"))
                    .shortcut(im_str!("CTRL+X"))
                    .build(ui);
                MenuItem::new(im_str!("Copy"))
                    .shortcut(im_str!("CTRL+C"))
                    .build(ui);
                MenuItem::new(im_str!("Paste"))
                    .shortcut(im_str!("CTRL+V"))
                    .build(ui);
                menu.end(ui);
            }
            menu_bar.end(ui);
        }
        if self.show_object_info {
            let mut show_window = self.show_object_info;
            Window::new(im_str!("Object info"))
                .size([250.0, 250.0], Condition::FirstUseEver)
                .opened(&mut show_window)
                .build(&ui, || {
                    let mut selected_mut = vec![&mut self.model.transform];
                    <Transform as imgui_inspect::InspectRenderStruct<Transform>>::render_mut(
                        &mut selected_mut,
                        "Object info",
                        &ui,
                        &InspectArgsStruct::default(),
                    );
                });
            self.show_object_info = show_window;
        }
    }

    fn on_mouse_scroll(&mut self, yoffset: f32) {
        self.camera.process_mouse_scroll(yoffset as f32);
    }
    fn on_mouse_move(&mut self, x: f32, y: f32) {
        self.camera.process_mouse_movement(x, y, true);
    }
}
