use doppler::assets_cache::AssetsCache;
use doppler::camera::*;
use doppler::client::Client;
use doppler::components::{ModelComponent, Transform};
use doppler::consts;
use doppler::glutin::event::{ElementState, VirtualKeyCode};
use doppler::imgui::*;
use doppler::light::*;
use doppler::map::*;
use doppler::math::prelude::*;
use doppler::math::{perspective, vec3, Deg, Matrix4, Point3};
use doppler::sky::Sky;

pub struct ExampleClient {
    map: Map,
    delta: f32,
    show_object_info: bool,
    show_camera_info: bool,
    object_info_id: i32,
    show_light_info: bool,
    light_info_id: i32,
}

impl Default for ExampleClient {
    fn default() -> Self {
        ExampleClient {
            delta: 0.0,
            map: Map::default(),
            object_info_id: 0,
            show_camera_info: false,
            show_object_info: false,
            show_light_info: false,
            light_info_id: 0,
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
        MapSave::save(&self.map, "test");
    }

    unsafe fn draw(&mut self) {
        self.map.draw();
    }
    fn update(&mut self, delta: f32) {
        self.delta = delta;
    }

    fn debug_draw(&mut self, ui: &doppler::imgui::Ui) {
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

        {
            use doppler::imgui_inspect;

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
                            &[&self.map.camera],
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
                let max: i32 = self.map.models.len() as i32 - 1;
                let mut show_window = self.show_object_info;

                Window::new(im_str!("Object info"))
                    .size([250.0, 250.0], Condition::FirstUseEver)
                    .opened(&mut show_window)
                    .build(&ui, || {
                        ui.input_int(im_str!("id"), &mut id).build();
                        id = if id < 0 {
                            0
                        } else if id > max {
                            max
                        } else {
                            id
                        };

                        let mut selected_mut = vec![&mut self.map.models[id as usize].transform];
                        <Transform as imgui_inspect::InspectRenderStruct<Transform>>::render_mut(
                            &mut selected_mut,
                            "Object info",
                            &ui,
                            &imgui_inspect::InspectArgsStruct::default(),
                        );
                    });
                self.object_info_id = id;
                self.show_object_info = show_window;
            }
            if self.show_light_info {
                let mut id = self.light_info_id;
                let max: i32 = self.map.lighting_system.point_lights.len() as i32 - 1;
                let mut show_window = self.show_light_info;

                Window::new(im_str!("Lights info"))
                .size([250.0, 250.0], Condition::FirstUseEver)
                .opened(&mut show_window)
                .build(&ui, || {
                    {
                        let mut selected_mut = vec![&mut self.map.lighting_system.directional_light];
                        <DirectionalLight as imgui_inspect::InspectRenderStruct<
                            DirectionalLight,
                        >>::render_mut(
                            &mut selected_mut,
                            "DirectionalLightInfo",
                            &ui,
                            &imgui_inspect::InspectArgsStruct::default(),
                        );
                    }
                    ui.separator();
                    ui.input_int(im_str!("Light ID"), &mut id)
                        .build();
                    id = if id < 0 { 0 } else if id > max { max } else { id };
                    {
                        let mut selected_mut =
                            vec![&mut self.map.lighting_system.point_lights[id as usize]];
                        <PointLight as imgui_inspect::InspectRenderStruct<PointLight>>::render_mut(
                            &mut selected_mut,
                            "PointLightInfo",
                            &ui,
                            &imgui_inspect::InspectArgsStruct::default(),
                        );
                    }
                });

                self.light_info_id = id;
                self.show_light_info = show_window;
            }
        }
    }

    fn on_mouse_scroll(&mut self, yoffset: f32) {
        self.map.camera.process_mouse_scroll(yoffset as f32);
    }
    fn on_mouse_move(&mut self, x: f32, y: f32) {
        self.map.camera.process_mouse_movement(x, y, true);
    }
}
