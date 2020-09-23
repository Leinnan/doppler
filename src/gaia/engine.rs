use crate::gaia::bg_info::BgInfo;
use crate::gaia::camera::*;
use crate::gaia::consts;
use crate::gaia::*;
use crate::example_client::ExampleClient;
use crate::gaia::client::Client;
use cgmath::{perspective, vec3, Deg, Matrix4, Point3, Rad, Vector3};
use imgui_glfw_rs::glfw;
use imgui_glfw_rs::glfw::{Action, Context, Key};
use imgui_glfw_rs::imgui;
use imgui_glfw_rs::ImguiGLFW;
use imgui_inspect::InspectArgsStruct;

pub struct Engine {
    pub camera: Camera,
    pub bg_info: BgInfo,
    pub window: imgui_glfw_rs::glfw::Window,
    pub window_size: (f32, f32),
    pub events: std::sync::mpsc::Receiver<(f64, imgui_glfw_rs::glfw::WindowEvent)>,
    pub glfw: imgui_glfw_rs::glfw::Glfw,
    pub imgui: imgui::Context,
    pub imgui_glfw: ImguiGLFW,
    pub shader: shader::Shader,
    pub models: Vec<model::Model>,
    pub client: Box<dyn Client>,
}

impl Engine {
    pub fn run(&mut self) {
        let mut first_mouse = true;
        let mut last_x: f32 = consts::SCR_WIDTH as f32 / 2.0;
        let mut last_y: f32 = consts::SCR_HEIGHT as f32 / 2.0;

        // timing
        let mut delta_time: f32; // time between current frame and last frame
        let mut last_frame: f32 = 0.0;
        {
            // build and compile shaders
            // -------------------------
            self.shader = shader::Shader::from_file(
                "resources/shaders/model_loading.vs",
                "resources/shaders/model_loading.fs",
            );

            // load models
            // -----------
            for _ in 0..10 {
                self.models.push(model::Model::new("resources/objects/nanosuit/nanosuit.obj"));   
            }
        };
        // render loop
        // -----------
        while !self.window.should_close() {
            // per-frame time logic
            // --------------------
            let cur_frame = self.glfw.get_time() as f32;
            delta_time = cur_frame - last_frame;
            last_frame = cur_frame;

            // input
            // -----
            let skip_input =
                self.imgui.io().want_capture_mouse || self.imgui.io().want_capture_keyboard;
            if !skip_input {
                self.process_input(delta_time);
            }

            // render
            // ------
            unsafe {
                gl::ClearColor(self.bg_info.r, self.bg_info.g, self.bg_info.b, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

                // don't forget to enable shader before setting uniforms
                self.shader.useProgram();

                // view/projection transformations
                let projection: Matrix4<f32> = perspective(
                    Deg(self.camera.Zoom),
                    self.window_size.0 / self.window_size.1,
                    0.1,
                    100.0,
                );
                let view = self.camera.get_view_matrix();
                self.shader.setMat4(c_str!("projection"), &projection);
                self.shader.setMat4(c_str!("view"), &view);

                let mut i = 0;
                for m in &self.models {
                    // render the loaded model
                    let mut model = Matrix4::<f32>::from_translation(vec3(0.0, -1.75, -1.25 * (i as f32))); // translate it down so it's at the center of the scene
                    model = model * Matrix4::from_scale(0.2); // it's a bit too big for our scene, so scale it down
                    self.shader.setMat4(c_str!("model"), &model);
                    m.Draw(&self.shader);
                    i = i + 1;
                }
                self.client.draw();
            }

            // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
            // -------------------------------------------------------------------------------

            let ui = self.imgui_glfw.frame(&mut self.window, &mut self.imgui);

            {
                use imgui::*;
                Window::new(im_str!("Hello world"))
                    .size([300.0, 110.0], Condition::FirstUseEver)
                    .build(&ui, || {
                        ui.text(im_str!("Hello world!"));
                        ui.text(im_str!("こんにちは世界！"));
                        ui.text(im_str!("This...is...imgui-rs!"));
                        ui.separator();
                        ui.text(format!("Mouse position: ({:.1},{:.1})", last_x, last_y));
                        // let selected = vec![&self.bg_info];
                        // <BgInfo as imgui_inspect::InspectRenderStruct<BgInfo>>::render(
                        //     &selected,
                        //     "Example Struct - Read Only",
                        //     &ui,
                        //     &InspectArgsStruct::default(),
                        // );
                        // let mut selected_mut = vec![&mut self.bg_info];
                        // <BgInfo as imgui_inspect::InspectRenderStruct<BgInfo>>::render_mut(
                        //     &mut selected_mut,
                        //     "Example Struct - Writable",
                        //     &ui,
                        //     &InspectArgsStruct::default(),
                        // );
                    });
            }

            self.imgui_glfw.draw(ui, &mut self.window);
            self.window.swap_buffers();
            self.glfw.poll_events();
            // events
            // -----
            self.process_events(&mut first_mouse, &mut last_x, &mut last_y, skip_input);
        }
    }

    pub fn process_input(&mut self, delta_time: f32) {
        if self.window.get_key(Key::Escape) == Action::Press {
            self.window.set_should_close(true)
        }
        if self.window.get_key(Key::W) == Action::Press {
            self.camera
                .process_keyboard(Camera_Movement::FORWARD, delta_time);
        }
        if self.window.get_key(Key::S) == Action::Press {
            self.camera
                .process_keyboard(Camera_Movement::BACKWARD, delta_time);
        }
        if self.window.get_key(Key::A) == Action::Press {
            self.camera
                .process_keyboard(Camera_Movement::LEFT, delta_time);
        }
        if self.window.get_key(Key::D) == Action::Press {
            self.camera
                .process_keyboard(Camera_Movement::RIGHT, delta_time);
        }
        self.camera
            .enable_mouse_movement(self.window.get_key(Key::LeftControl) != Action::Press);
    }

    pub fn process_events(
        &mut self,
        first_mouse: &mut bool,
        last_x: &mut f32,
        last_y: &mut f32,
        skip_input: bool,
    ) {
        for (_, event) in glfw::flush_messages(&self.events) {
            self.imgui_glfw.handle_event(&mut self.imgui, &event);
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    // make sure the viewport matches the new window dimensions; note that width and
                    // height will be significantly larger than specified on retina displays.
                    unsafe { gl::Viewport(0, 0, width, height) }
                    self.window_size = (width as f32, height as f32);
                }
                glfw::WindowEvent::CursorPos(xpos, ypos) => {
                    if skip_input {
                        return;
                    }
                    let (xpos, ypos) = (xpos as f32, ypos as f32);
                    if *first_mouse {
                        *last_x = xpos;
                        *last_y = ypos;
                        *first_mouse = false;
                    }

                    let xoffset = xpos - *last_x;
                    let yoffset = *last_y - ypos; // reversed since y-coordinates go from bottom to top

                    *last_x = xpos;
                    *last_y = ypos;

                    self.camera.process_mouse_movement(xoffset, yoffset, true);
                }
                glfw::WindowEvent::Scroll(_xoffset, yoffset) => {
                    if skip_input {
                        return;
                    }
                    self.camera.process_mouse_scroll(yoffset as f32);
                }
                _ => {}
            }
        }
    }
}

impl Default for Engine {
    fn default() -> Self {
        // glfw: initialize and configure
        // ------------------------------
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        #[cfg(target_os = "macos")]
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        // glfw window creation
        // --------------------
        let (mut window, events) = glfw
            .create_window(
                consts::SCR_WIDTH,
                consts::SCR_HEIGHT,
                "chRustedGL",
                glfw::WindowMode::Windowed,
            )
            .expect("Failed to create GLFW window");

        window.make_current();
        window.set_all_polling(true);
        window.set_framebuffer_size_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_scroll_polling(true);

        // tell GLFW to capture our mouse
        window.set_cursor_mode(glfw::CursorMode::Disabled);
        // gl: load all OpenGL function pointers
        // ---------------------------------------
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        let mut imgui = imgui::Context::create();
        let mut imgui_glfw = ImguiGLFW::new(&mut imgui, &mut window);
        // configure global opengl state
        // -----------------------------
        unsafe{gl::Enable(gl::DEPTH_TEST);}

        Engine {
            bg_info: BgInfo::default(),
            window: window,
            window_size: (consts::SCR_WIDTH as f32, consts::SCR_HEIGHT as f32),
            events: events,
            glfw: glfw,
            imgui: imgui,
            imgui_glfw: imgui_glfw,
            camera: Camera {
                position: Point3::new(0.0, 0.0, 3.0),
                ..Camera::default()
            },
            shader: shader::Shader::default(),
            models: vec![],
            client: Box::new(ExampleClient::create()),
        }
    }
}
