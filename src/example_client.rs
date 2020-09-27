use crate::gaia::assets_cache::AssetsCache;
use crate::gaia::camera::*;
use crate::gaia::client::Client;
use crate::gaia::components::{ModelComponent, Transform};
use crate::gaia::consts;
use crate::gaia::engine::Engine;
use crate::gaia::light::*;
use crate::gaia::sky::Sky;
use crate::gaia::*;
use cgmath::prelude::*;
use cgmath::{perspective, vec3, Deg, Matrix4, Point3, Vector3};
use imgui_glfw_rs::glfw;

pub struct ExampleClient {
    models: Vec<ModelComponent>,
    camera: Camera,
    lighting_system: LightingSystem,
    sky: Sky,
    show_object_info: bool,
    show_camera_info: bool,
    object_info_id: i32,
    show_light_info: bool,
    light_info_id: i32,
}

impl ExampleClient {
    pub fn create(_window: &glfw::Window) -> ExampleClient {
        let sky = unsafe { Sky::new() };
        ExampleClient {
            models: vec![],
            camera: Camera {
                position: Point3::new(0.0, 8.0, 13.0),
                front: vec3(0.0, -0.4, -1.0),
                up: vec3(0.0, 1.0, -0.4),
                right: vec3(1.0, 0.0, 0.0),
                yaw: -90.0,
                pitch: -20.0,
                ..Camera::default()
            },
            lighting_system: LightingSystem::default(),
            sky: sky,
            object_info_id: 0,
            show_camera_info: false,
            show_object_info: false,
            show_light_info: false,
            light_info_id: 0,
        }
    }
}

impl Client for ExampleClient {
    fn load_assets(&mut self, cache: &mut AssetsCache) {
        let ground = ModelComponent {
            transform: Transform {
                scale: 0.56,
                ..Transform::default()
            },
            model: cache.get_model("resources/objects/ground/ground.obj"),
        };
        let robot = ModelComponent {
            transform: Transform::default(),
            model: cache.get_model("resources/objects/robot/robot.obj"),
        };
        let grass = ModelComponent {
            transform: Transform::default(),
            model: cache.get_model_ext("resources/objects/grass/grass.obj", Some("foliage.png")),
        };
        let gaz_tank = ModelComponent {
            transform: Transform {
                position: vec3(8.3, 0.0, 3.0),
                rotation: vec3(0.0,120.0,0.0),
                scale: 0.025,
                ..Transform::default()
            },
            model: cache.get_model("resources/objects/gaz_tank/gaz_tank.obj"),
        };
        let tree = ModelComponent {
            transform: Transform {
                position: vec3(10.0, 0.0, 26.0),
                scale: 2.5,
                ..Transform::default()
            },
            model: cache.get_model_ext("resources/objects/tree/tree_6_d.obj", Some("tree_e.png")),
        };
        let tree2 = ModelComponent {
            transform: Transform {
                position: vec3(-9.0, 0.0, -15.0),
                scale: 2.5,
                ..Transform::default()
            },
            model: cache.get_model_ext("resources/objects/tree/tree_6_c.obj", Some("tree_e.png")),
        };
        let tree3 = ModelComponent {
            transform: Transform {
                position: vec3(15.0, 0.0, -7.0),
                scale: 2.5,
                ..Transform::default()
            },
            model: cache.get_model_ext("resources/objects/tree/tree_6_c.obj", Some("tree_e.png")),
        };
        let ruins = ModelComponent {
            transform: Transform::default(),
            model: cache.get_model("resources/objects/ruins/ruins.obj"),
        };
        self.models = vec![tree, tree2, tree3, ground, robot, ruins,gaz_tank, grass];
    }

    unsafe fn draw(&mut self) {
        // view/projection transformations
        let projection: Matrix4<f32> = perspective(
            Deg(self.camera.zoom),
            consts::SCR_WIDTH as f32 / consts::SCR_HEIGHT as f32,
            0.1,
            1000.0,
        );
        let view = self.camera.get_view_matrix();
        let view_pos = self.camera.position.to_vec();
        self.lighting_system
            .prepare_for_draw(&projection, &view, &view_pos);

        for model in self.models.iter() {
            model.draw(&self.lighting_system.shader);
        }
        self.sky.draw(view, projection);
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
                if MenuItem::new(im_str!("Show lights info"))
                    .selected(self.show_light_info)
                    .build(ui)
                {
                    self.show_light_info = !self.show_light_info;
                }
                menu.end(ui);
            }
            if MenuItem::new(im_str!("Toggle Camera Info"))
                .selected(self.show_camera_info)
                .build(ui)
            {
                self.show_camera_info = !self.show_camera_info;
            }
            menu_bar.end(ui);
        }
        if self.show_camera_info {
            Window::new(im_str!("CameraInfo"))
                .size([260.0, 430.0], Condition::Always)
                .position([20.0, 40.0], Condition::Always)
                .title_bar(false)
                .scroll_bar(false)
                .no_inputs()
                .bg_alpha(0.8)
                .collapsible(false)
                .build(&ui, || {
                    ui.text(im_str!("Camera info"));
                    ui.separator();
                    <Camera as imgui_inspect::InspectRenderDefault<Camera>>::render(
                        &[&self.camera],
                        &"CameraInfo",
                        ui,
                        &imgui_inspect::InspectArgsDefault {
                            header: Some(false),
                            ..imgui_inspect::InspectArgsDefault::default()
                        },
                    );
                });
        }
        if self.show_object_info {
            let mut id = self.object_info_id;
            let max: i32 = self.models.len() as i32 - 1;
            let mut show_window = self.show_object_info;

            Window::new(im_str!("Object info"))
                .size([250.0, 250.0], Condition::FirstUseEver)
                .opened(&mut show_window)
                .build(&ui, || {
                    ui.drag_int(im_str!("id"), &mut id).min(0).max(max).build();

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
        if self.show_light_info {
            let mut id = self.light_info_id;
            let max: i32 = self.lighting_system.point_lights.len() as i32 - 1;
            let mut show_window = self.show_light_info;

            Window::new(im_str!("Lights info"))
                .size([250.0, 250.0], Condition::FirstUseEver)
                .opened(&mut show_window)
                .build(&ui, || {
                    {
                        let mut selected_mut = vec![&mut self.lighting_system.directional_light];
                        <DirectionalLight as imgui_inspect::InspectRenderStruct<
                            DirectionalLight,
                        >>::render_mut(
                            &mut selected_mut,
                            "DirectionalLightInfo",
                            &ui,
                            &InspectArgsStruct::default(),
                        );
                    }
                    ui.separator();
                    ui.drag_int(im_str!("Light ID"), &mut id)
                        .min(0)
                        .max(max)
                        .build();
                    {
                        let mut selected_mut =
                            vec![&mut self.lighting_system.point_lights[id as usize]];
                        <PointLight as imgui_inspect::InspectRenderStruct<PointLight>>::render_mut(
                            &mut selected_mut,
                            "PointLightInfo",
                            &ui,
                            &InspectArgsStruct::default(),
                        );
                    }
                });
            self.light_info_id = id;
            self.show_light_info = show_window;
        }
    }

    fn on_mouse_scroll(&mut self, yoffset: f32) {
        self.camera.process_mouse_scroll(yoffset as f32);
    }
    fn on_mouse_move(&mut self, x: f32, y: f32) {
        self.camera.process_mouse_movement(x, y, true);
    }
}
