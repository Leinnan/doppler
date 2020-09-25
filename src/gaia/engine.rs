use crate::example_client::ExampleClient;
use crate::gaia::bg_info::BgInfo;
use crate::gaia::camera::*;
use crate::gaia::client::Client;
use crate::gaia::consts;
use crate::gaia::assets_cache::AssetsCache;
use cgmath::Point3;
use imgui_glfw_rs::glfw;
use imgui_glfw_rs::glfw::{Action, Context, Key};
use imgui_glfw_rs::imgui;
use imgui_glfw_rs::ImguiGLFW;

pub struct Engine {
    pub camera: Camera,
    pub bg_info: BgInfo,
    pub window: imgui_glfw_rs::glfw::Window,
    pub window_size: (f32, f32),
    pub events: std::sync::mpsc::Receiver<(f64, imgui_glfw_rs::glfw::WindowEvent)>,
    pub glfw: imgui_glfw_rs::glfw::Glfw,
    pub imgui: imgui::Context,
    pub imgui_glfw: ImguiGLFW,
    pub client: Box<dyn Client>,
    enable_debug_layer: bool,
    assets_cache: AssetsCache,
}

impl Engine {
    pub fn run(&mut self) {
        self.client.load_assets(&mut self.assets_cache);
        let mut first_mouse = true;
        let mut last_x: f32 = consts::SCR_WIDTH as f32 / 2.0;
        let mut last_y: f32 = consts::SCR_HEIGHT as f32 / 2.0;

        // timing
        let mut delta_time: f32; // time between current frame and last frame
        let mut last_frame: f32 = 0.0;
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

                self.client.draw();
            }

            // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
            // -------------------------------------------------------------------------------

            if self.enable_debug_layer {
                let ui = self.imgui_glfw.frame(&mut self.window, &mut self.imgui);
                self.client.debug_draw(&ui);
                use imgui::*;
                let fps = 1.0 / delta_time;
                let size = [250.0, 110.0];
                let offset = 20.0;
                Window::new(im_str!("EngineInfo"))
                    .size(size, Condition::Always)
                    .position(
                        [
                            self.window_size.0 - size[0] - offset,
                            self.window_size.1 - size[1] - offset,
                        ],
                        Condition::Always,
                    )
                    .no_decoration()
                    .no_inputs()
                    .save_settings(false)
                    .build(&ui, || {
                        ui.text("Welcome in doppler world!");
                        ui.text(format!("FPS: {:.0}", fps));
                        ui.separator();
                        ui.text(format!("Mouse position: ({:4.1},{:4.1})", last_x, last_y));
                    });
                self.imgui_glfw.draw(ui, &mut self.window);
            }

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
        if self.window.get_key(Key::P) == Action::Press
            && self.window.get_key(Key::LeftShift) == Action::Press
        {
            self.enable_debug_layer = !self.enable_debug_layer;
        }
        self.client.process_input(&self.window, delta_time);
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

                    self.client.on_mouse_move(xoffset, yoffset);
                }
                glfw::WindowEvent::Scroll(_xoffset, yoffset) => {
                    if skip_input {
                        return;
                    }
                    self.client.on_mouse_scroll(yoffset as f32);
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
        {
            use imgui_glfw_rs::imgui::FontSource;
            use imgui_glfw_rs::imgui::StyleColor;
            let mut style = imgui.style_mut();
            style.scale_all_sizes(1.5);
            style[StyleColor::Text] = [1.0, 1.0, 1.0, 1.0];
            style[StyleColor::TextDisabled] = [0.5, 0.5, 0.5, 1.0];
            style[StyleColor::WindowBg] = [0.13, 0.14, 0.15, 1.0];
            style[StyleColor::ChildBg] = [0.13, 0.14, 0.15, 1.0];
            style[StyleColor::PopupBg] = [0.13, 0.14, 0.15, 1.0];
            style[StyleColor::Border] = [0.43, 0.43, 0.50, 0.50];
            style[StyleColor::BorderShadow] = [0.00, 0.00, 0.00, 0.00];
            style[StyleColor::FrameBg] = [0.25, 0.25, 0.25, 1.00];
            style[StyleColor::FrameBgHovered] = [0.38, 0.38, 0.38, 1.00];
            style[StyleColor::FrameBgActive] = [0.67, 0.67, 0.67, 0.39];
            style[StyleColor::TitleBg] = [0.08, 0.08, 0.09, 1.00];
            style[StyleColor::TitleBgActive] = [0.08, 0.08, 0.09, 1.00];
            style[StyleColor::TitleBgCollapsed] = [0.00, 0.00, 0.00, 0.51];
            style[StyleColor::MenuBarBg] = [0.14, 0.14, 0.14, 1.00];
            style[StyleColor::ScrollbarBg] = [0.02, 0.02, 0.02, 0.53];
            style[StyleColor::ScrollbarGrab] = [0.31, 0.31, 0.31, 1.00];
            style[StyleColor::ScrollbarGrabHovered] = [0.41, 0.41, 0.41, 1.00];
            style[StyleColor::ScrollbarGrabActive] = [0.51, 0.51, 0.51, 1.00];
            style[StyleColor::CheckMark] = [0.11, 0.64, 0.92, 1.00];
            style[StyleColor::SliderGrab] = [0.11, 0.64, 0.92, 1.00];
            style[StyleColor::SliderGrabActive] = [0.08, 0.50, 0.72, 1.00];
            style[StyleColor::Button] = [0.25, 0.25, 0.25, 1.00];
            style[StyleColor::ButtonHovered] = [0.38, 0.38, 0.38, 1.00];
            style[StyleColor::ButtonActive] = [0.67, 0.67, 0.67, 0.39];
            style[StyleColor::Header] = [0.22, 0.22, 0.22, 1.00];
            style[StyleColor::HeaderHovered] = [0.25, 0.25, 0.25, 1.00];
            style[StyleColor::HeaderActive] = [0.67, 0.67, 0.67, 0.39];
            style[StyleColor::Separator] = style[StyleColor::Border];
            style[StyleColor::SeparatorHovered] = [0.41, 0.42, 0.44, 1.00];
            style[StyleColor::SeparatorActive] = [0.26, 0.59, 0.98, 0.95];
            style[StyleColor::ResizeGrip] = [0.00, 0.00, 0.00, 0.00];
            style[StyleColor::ResizeGripHovered] = [0.29, 0.30, 0.31, 0.67];
            style[StyleColor::ResizeGripActive] = [0.26, 0.59, 0.98, 0.95];
            style[StyleColor::Tab] = [0.08, 0.08, 0.09, 0.83];
            style[StyleColor::TabHovered] = [0.33, 0.34, 0.36, 0.83];
            style[StyleColor::TabActive] = [0.23, 0.23, 0.24, 1.00];
            style[StyleColor::TabUnfocused] = [0.08, 0.08, 0.09, 1.00];
            style[StyleColor::TabUnfocusedActive] = [0.13, 0.14, 0.15, 1.00];
            // style[StyleColor::DockingPreview]        = [0.26, 0.59, 0.98, 0.70];
            // style[StyleColor::DockingEmptyBg]        = [0.20, 0.20, 0.20, 1.00];
            style[StyleColor::PlotLines] = [0.61, 0.61, 0.61, 1.00];
            style[StyleColor::PlotLinesHovered] = [1.00, 0.43, 0.35, 1.00];
            style[StyleColor::PlotHistogram] = [0.90, 0.70, 0.00, 1.00];
            style[StyleColor::PlotHistogramHovered] = [1.00, 0.60, 0.00, 1.00];
            style[StyleColor::TextSelectedBg] = [0.26, 0.59, 0.98, 0.35];
            style[StyleColor::DragDropTarget] = [0.11, 0.64, 0.92, 1.00];
            style[StyleColor::NavHighlight] = [0.26, 0.59, 0.98, 1.00];
            style[StyleColor::NavWindowingHighlight] = [1.00, 1.00, 1.00, 0.70];
            style[StyleColor::NavWindowingDimBg] = [0.80, 0.80, 0.80, 0.20];
            style[StyleColor::ModalWindowDimBg] = [0.80, 0.80, 0.80, 0.35];
            style.grab_rounding = 2.3;
            style.frame_rounding = style.grab_rounding;
            imgui.fonts().clear();
            imgui.fonts().add_font(&[FontSource::TtfData {
                data: include_bytes!("../../resources/FiraSans-SemiBold.ttf"),
                size_pixels: 19.0,
                config: None,
            }]);
        }
        let imgui_glfw = ImguiGLFW::new(&mut imgui, &mut window);
        // configure global opengl state
        // -----------------------------
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
        }

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
            enable_debug_layer: true,
            client: Box::new(ExampleClient::create()),
            assets_cache: AssetsCache::default(),
        }
    }
}
