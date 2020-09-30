use crate::assets_cache::AssetsCache;
use crate::client::Client;
use crate::consts;
use crate::framebuffer::FramebufferSystem;
use glutin::event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent};
#[cfg(feature = "imgui_inspect")]
use imgui::Context;
#[cfg(feature = "imgui_inspect")]
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use std::time::Instant;
use log::info;
use log::LevelFilter;

#[derive(Debug)]
pub struct TimeStep {
    last_time: Instant,
    delta_time: f32,
    frame_count: u32,
    frame_time: f32,
    last_frame_count: u32,
}

impl TimeStep {
    pub fn new() -> TimeStep {
        TimeStep {
            last_time: Instant::now(),
            delta_time: 0.0,
            frame_count: 0,
            frame_time: 0.0,
            last_frame_count: 42,
        }
    }

    pub fn update(&mut self) {
        // delta
        let current_time = Instant::now();
        let delta = current_time.duration_since(self.last_time).as_micros() as f32 * 0.001;
        self.last_time = current_time;
        self.delta_time = delta;

        // frame counter
        self.frame_count += 1;
        self.frame_time += self.delta_time;
        // per second
        if self.frame_time >= 1000.0 {
            self.last_frame_count = self.frame_count;
            self.frame_count = 0;
            self.frame_time = 0.0;
        }
    }

    pub fn delta(&self) -> f32 {
        self.delta_time
    }

    // provides the framerate in FPS
    pub fn frame_rate(&self) -> u32 {
        self.last_frame_count
    }
}

pub struct Engine {
    title: String,
    size: (i32, i32),
    #[cfg(feature = "imgui_inspect")]
    debug_layer: bool,
}

impl Default for Engine {
    fn default() -> Self {
        Self {
            title: String::from("Doppler demo"),
            size: (1280, 720),
            #[cfg(feature = "imgui_inspect")]
            debug_layer: true,
        }
    }
}

impl Engine {
    pub fn run<T: Client + Default + 'static>(self) {
        let _ = simple_logging::log_to_file("log.log", LevelFilter::Info);
        info!("Starting engine!");
        let event_loop = glutin::event_loop::EventLoop::new();
        let window = glutin::window::WindowBuilder::new()
            .with_title(&self.title)
            .with_inner_size(glutin::dpi::PhysicalSize::new(
                self.size.0 as f32,
                self.size.1 as f32,
            ))
            .with_resizable(true);
        let gl_window = glutin::ContextBuilder::new()
            .build_windowed(window, &event_loop)
            .unwrap();

        let gl_window = unsafe { gl_window.make_current().unwrap() };
        info!(
            "Pixel format of the window's GL context: {:?}",
            gl_window.get_pixel_format()
        );

        gl::load_with(|symbol| gl_window.get_proc_address(symbol));
        // configure global opengl state
        // -----------------------------
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
        }
        info!("DPI: {}", gl_window.window().scale_factor());
        let mut framebuffer = unsafe { FramebufferSystem::generate(self.size.0, self.size.1) };
        #[cfg(feature = "imgui_inspect")]
        let (mut imgui, mut platform, renderer) = {
            let mut imgui = Context::create();
            use imgui::{FontSource, StyleColor};
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
                data: include_bytes!("../resources/FiraSans-SemiBold.ttf"),
                size_pixels: 19.0,
                config: None,
            }]);
            info!("Fonts amount int imgui: {}", imgui.fonts().fonts().len());
            let mut platform = WinitPlatform::init(&mut imgui); // step 1
            platform.attach_window(imgui.io_mut(), &gl_window.window(), HiDpiMode::Locked(1.0)); // step 2
            let renderer = imgui_opengl_renderer::Renderer::new(&mut imgui, |s| {
                gl_window.get_proc_address(s) as _
            });
            (imgui, platform, renderer)
        };
        info!("Creating AssetCache");
        let mut assets_cache = AssetsCache::default();
        info!("Creating client");
        let mut client = Box::new(T::default());
        client.load_assets(&mut assets_cache);
        let mut first_mouse = true;
        let mut last_x: f32 = consts::SCR_WIDTH as f32 / 2.0;
        let mut last_y: f32 = consts::SCR_HEIGHT as f32 / 2.0;

        // timing
        let mut timestep = TimeStep::new();
        let mut last_frame = std::time::Instant::now();

        let mut screensize = self.size;

        event_loop.run(move |event, _, control_flow| {
            use glutin::event_loop::ControlFlow;
            *control_flow = ControlFlow::Poll;
            #[cfg(feature = "imgui_inspect")]
            platform.handle_event(imgui.io_mut(), &gl_window.window(), &event);
            match event {
                Event::NewEvents(_) => {
                    // other application-specific logic
                    #[cfg(feature = "imgui_inspect")]
                    {
                        last_frame = imgui.io_mut().update_delta_time(last_frame);
                    }
                }
                Event::MainEventsCleared => {
                    timestep.update();
                    client.update(timestep.delta());
                    // other application-specific logic
                    #[cfg(feature = "imgui_inspect")]
                    platform
                        .prepare_frame(imgui.io_mut(), &gl_window.window()) // step 4
                        .expect("Failed to prepare frame");
                    gl_window.window().request_redraw();
                }
                Event::LoopDestroyed => return,
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::MouseWheel { delta, .. } => {
                        use glutin::event::MouseScrollDelta;
                        match delta {
                            MouseScrollDelta::LineDelta(_, y) => client.on_mouse_scroll(y),
                            MouseScrollDelta::PixelDelta(pos) => {
                                client.on_mouse_scroll(pos.y as f32)
                            }
                        }
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        let xpos = position.x as f32;
                        let ypos = position.y as f32;
                        if first_mouse {
                            last_x = xpos;
                            last_y = ypos;
                            first_mouse = false;
                        }

                        let xoffset = xpos - last_x;
                        let yoffset = last_y - ypos; // reversed since y-coordinates go from bottom to top

                        last_x = xpos;
                        last_y = ypos;

                        client.on_mouse_move(xoffset, yoffset);
                    }
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(virtual_code),
                                state,
                                ..
                            },
                        ..
                    } => match (virtual_code, state) {
                        (VirtualKeyCode::Escape, _) => *control_flow = ControlFlow::Exit,
                        _ => client.on_keyboard(&virtual_code, &state),
                    },
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(size) => {
                        info!("Resizing to {:?}", size);
                        unsafe { gl::Viewport(0, 0, size.width as i32, size.height as i32) }
                        screensize = (size.width as i32, size.height as i32);
                        framebuffer = unsafe {
                            FramebufferSystem::generate(size.width as i32, size.height as i32)
                        };

                        #[cfg(feature = "imgui_inspect")]
                        platform.attach_window(
                            imgui.io_mut(),
                            &gl_window.window(),
                            HiDpiMode::Locked(1.0),
                        );
                    }
                    _ => (),
                },
                Event::RedrawRequested(_) => {
                    unsafe {
                        framebuffer.clear();
                        client.draw();
                        framebuffer.draw();
                    }

                    #[cfg(feature = "imgui_inspect")]
                    if self.debug_layer {
                        imgui.io_mut().display_size = [screensize.0 as f32, screensize.1 as f32];
                        let imgui_size = imgui.io().display_size;
                        let ui = imgui.frame();
                        client.debug_draw(&ui);
                        use imgui::*;
                        let fps = timestep.frame_rate();
                        let size = [250.0, 110.0];
                        let offset = 20.0;
                        Window::new(im_str!("EngineInfo"))
                            .size(size, Condition::Always)
                            .position(
                                [
                                    imgui_size[0] - size[0] - offset,
                                    imgui_size[1] - size[1] - offset,
                                ],
                                Condition::Always,
                            )
                            .no_decoration()
                            .no_inputs()
                            .bg_alpha(0.8)
                            .save_settings(false)
                            .build(&ui, || {
                                ui.text("Welcome in doppler world!");
                                ui.text(format!("FPS: {:.0}", fps));
                                ui.separator();
                                ui.text(format!("Mouse position: ({:4.1},{:4.1})", last_x, last_y));
                            });
                        Window::new(im_str!("Logs"))
                            .size([670.0, 215.0], Condition::Always)
                            .bg_alpha(0.8)
                            .position(
                                [offset, imgui_size[1] as f32 - 215.0 - offset],
                                Condition::Always,
                            )
                            .no_decoration()
                            .no_inputs()
                            .bg_alpha(0.8)
                            .save_settings(false)
                            .build(&ui, || {
                                use std::fs;
                                use std::io::prelude::*;
                                use std::io::BufReader;
                                let buf = BufReader::new(
                                    fs::File::open("log.log").expect("no such file"),
                                );
                                let lines: Vec<String> = buf
                                    .lines()
                                    .map(|l| l.expect("Could not parse line"))
                                    .collect();
                                let mut output = String::new();
                                lines.iter().rev().take(10).rev().for_each(|line| {
                                    output.push_str(&line);
                                    output.push('\n');
                                });

                                ui.text(output);
                            });
                        platform.prepare_render(&ui, &gl_window.window());
                        renderer.render(ui);
                    }
                    gl_window.swap_buffers().unwrap();
                }
                _ => (),
            }
        });
    }
}
