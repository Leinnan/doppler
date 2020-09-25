use crate::gaia::camera::*;
use crate::gaia::client::Client;
use crate::gaia::components::{ModelComponent, Transform};
use crate::gaia::consts;
use crate::gaia::engine::Engine;
use crate::gaia::*;
use cgmath::{perspective, vec3, Deg, Matrix4, Point3};
use imgui_glfw_rs::glfw;

pub struct ExampleClient {
    models: Vec<ModelComponent>,
    camera: Camera,
    shader: shader::Shader,
    show_object_info: bool,
    show_camera_info: bool,
    object_info_id: i32,
}

impl ExampleClient {
    pub fn create() -> ExampleClient {
        let tree = ModelComponent {
            transform: Transform {
                position: vec3(10.0, 0.0, 26.0),
                scale: vec3(2.5, 2.5, 2.5),
                ..Transform::default()
            },
            model: model::Model::new_ext("resources/objects/tree/tree_6_d.obj", Some("tree_e.png")),
        };
        let tree2 = ModelComponent {
            transform: Transform {
                position: vec3(-9.0, 0.0, -15.0),
                scale: vec3(2.5, 2.5, 2.5),
                ..Transform::default()
            },
            model: model::Model::new_ext("resources/objects/tree/tree_6_c.obj", Some("tree_e.png")),
        };
        let tree3 = ModelComponent {
            transform: Transform {
                position: vec3(15.0, 0.0, -7.0),
                scale: vec3(2.5, 2.5, 2.5),
                ..Transform::default()
            },
            model: model::Model::new_ext("resources/objects/tree/tree_6_c.obj", Some("tree_e.png")),
        };
        let ground = ModelComponent {
            transform: Transform {
                scale: vec3(0.5, 0.5, 0.5),
                ..Transform::default()
            },
            model: model::Model::new("resources/objects/ground/ground.obj"),
        };
        let robot = ModelComponent {
            transform: Transform::default(),
            model: model::Model::new("resources/objects/robot/robot.obj"),
        };
        ExampleClient {
            object_info_id: 0,
            show_camera_info: true,
            show_object_info: false,
            models: vec![tree, tree2, tree3, ground, robot],
            camera: Camera {
                position: Point3::new(0.0, 8.0, 13.0),
                front: vec3(0.0, -0.4, -1.0),
                up: vec3(0.0, 1.0, -0.4),
                right: vec3(1.0, 0.0, 0.0),
                yaw: -90.0,
                pitch: -20.0,
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
            1000.0,
        );
        let view = self.camera.get_view_matrix();
        self.shader.set_mat4(c_str!("projection"), &projection);
        self.shader.set_mat4(c_str!("view"), &view);

        for model in self.models.iter() {
            model.draw(&self.shader);
        }
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
        if window.get_key(Key::O) == Action::Press {
            println!("{:?}", self.camera);
        }
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
                if MenuItem::new(im_str!("Show camera info"))
                    .selected(self.show_camera_info)
                    .build(ui)
                {
                    self.show_camera_info = !self.show_camera_info;
                }
                menu.end(ui);
            }
            menu_bar.end(ui);
        }
        if self.show_camera_info {
            let text = format!("{:?}", self.camera)
                .replace("{", "{\n")
                .replace("}", "\n}")
                .replace("],", "],\n");
            Window::new(im_str!("CameraInfo"))
                .size([300.0, 300.0], Condition::Always)
                .position([50.0, 50.0], Condition::Always)
                .title_bar(false)
                .scroll_bar(false)
                .collapsible(false)
                .build(&ui, || {
                    ui.text(im_str!("Camera info"));
                    ui.separator();
                    ui.text(text);
                });
        }
        if self.show_object_info {
            let mut id = self.object_info_id;
            let max: i32 = self.models.len() as i32 - 1;
            println!("max: {}", max);
            let mut show_window = self.show_object_info;
            Window::new(im_str!("Object info"))
                .size([250.0, 250.0], Condition::FirstUseEver)
                .opened(&mut show_window)
                .build(&ui, || {
                    ui.drag_int(im_str!("id"), &mut id).min(0).max(max).build();
                    // id = if id < 0 { 0 } else if id > max { max } else { id };
                    let mut selected_mut = vec![&mut self.models[id as usize].transform];
                    <Transform as imgui_inspect::InspectRenderStruct<Transform>>::render_mut(
                        &mut selected_mut,
                        "Object info",
                        &ui,
                        &InspectArgsStruct::default(),
                    );
                });
            self.object_info_id = id;
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
