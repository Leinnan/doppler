extern crate gl;
extern crate imgui_glfw_rs;
extern crate image;
// Use the reexported glfw crate to avoid version conflicts.
use imgui_glfw_rs::glfw;
// Use the reexported imgui crate to avoid version conflicts.
use imgui_glfw_rs::imgui;

use imgui_glfw_rs::ImguiGLFW;
use imgui_inspect::InspectArgsStruct;
use self::gl::types::*;
use imgui_glfw_rs::glfw::{Action, Context, Key};
use cgmath::prelude::*;
use cgmath::{perspective, vec3, Deg, Matrix4, Point3, Rad, Vector3};
use human_panic::setup_panic;

#[macro_use]
mod gaia;
use crate::gaia::camera::*;
use crate::gaia::*;
use crate::gaia::bg_info::BgInfo;

pub fn main() {
    setup_panic!();
    let mut camera = Camera {
        Position: Point3::new(0.0, 0.0, 3.0),
        ..Camera::default()
    };

    let mut first_mouse = true;
    let mut last_x: f32 = consts::SCR_WIDTH as f32 / 2.0;
    let mut last_y: f32 = consts::SCR_HEIGHT as f32 / 2.0;

    // timing
    let mut delta_time: f32; // time between current frame and last frame
    let mut last_frame: f32 = 0.0;
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

    
    let (ourShader, ourModel) = unsafe {
        // configure global opengl state
        // -----------------------------
        gl::Enable(gl::DEPTH_TEST);

        // build and compile shaders
        // -------------------------
        let ourShader = shader::Shader::from_file(
            "resources/shaders/model_loading.vs",
            "resources/shaders/model_loading.fs");

        // load models
        // -----------
        let ourModel = model::Model::new("resources/objects/nanosuit/nanosuit.obj");

        // draw in wireframe
        // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

        (ourShader, ourModel)
    };


    let mut imgui = imgui::Context::create();

    let mut imgui_glfw = ImguiGLFW::new(&mut imgui, &mut window);

    // render loop
    // -----------
    let mut bg = BgInfo::default();
    while !window.should_close() {
        // per-frame time logic
        // --------------------
        let cur_frame = glfw.get_time() as f32;
        delta_time = cur_frame - last_frame;
        last_frame = cur_frame;

        // input
        // -----
        let skip_input = imgui.io().want_capture_mouse || imgui.io().want_capture_keyboard;
        if !skip_input {
            process_input(&mut window, delta_time, &mut camera);
        }

        // render
        // ------
        unsafe {
            gl::ClearColor(bg.r, bg.g, bg.b, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            // don't forget to enable shader before setting uniforms
            ourShader.useProgram();

            // view/projection transformations
            let projection: Matrix4<f32> = perspective(Deg(camera.Zoom), consts::SCR_WIDTH as f32 / consts::SCR_HEIGHT as f32, 0.1, 100.0);
            let view = camera.get_view_matrix();
            ourShader.setMat4(c_str!("projection"), &projection);
            ourShader.setMat4(c_str!("view"), &view);

            // render the loaded model
            let mut model = Matrix4::<f32>::from_translation(vec3(0.0, -1.75, 0.0)); // translate it down so it's at the center of the scene
            model = model * Matrix4::from_scale(0.2);  // it's a bit too big for our scene, so scale it down
            ourShader.setMat4(c_str!("model"), &model);
            ourModel.Draw(&ourShader);
        }

        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
        // -------------------------------------------------------------------------------
        
        let ui = imgui_glfw.frame(&mut window, &mut imgui);

        {
            use imgui::*;
            Window::new(im_str!("Hello world"))
            .size([300.0, 110.0], Condition::FirstUseEver)
            .build(&ui, || {
                ui.text(im_str!("Hello world!"));
                ui.text(im_str!("こんにちは世界！"));
                ui.text(im_str!("This...is...imgui-rs!"));
                ui.separator();
                ui.text(format!(
                    "Mouse Position: ({:.1},{:.1})",
                    last_x, last_y
                ));
                let selected = vec![&bg];
                <BgInfo as imgui_inspect::InspectRenderStruct<
                    BgInfo,
                >>::render(
                    &selected,
                    "Example Struct - Read Only",
                    &ui,
                    &InspectArgsStruct::default(),
                );
                let mut selected_mut = vec![&mut bg];
                <BgInfo as imgui_inspect::InspectRenderStruct<
                BgInfo,
                >>::render_mut(
                    &mut selected_mut,
                    "Example Struct - Writable",
                    &ui,
                    &InspectArgsStruct::default(),
                );
            });
        }

        imgui_glfw.draw(ui, &mut window);
        window.swap_buffers();
        glfw.poll_events();
        // events
        // -----
        
        for (_, event) in glfw::flush_messages(&events) {
            imgui_glfw.handle_event(&mut imgui, &event);
            process_events(
                event,
                &mut first_mouse,
                &mut last_x,
                &mut last_y,
                &mut camera,
                skip_input
            );
        }   
    }
}

pub fn process_events(
    event: imgui_glfw_rs::glfw::WindowEvent,
    first_mouse: &mut bool,
    last_x: &mut f32,
    last_y: &mut f32,
    camera: &mut Camera,
    skip_input: bool
) {
    match event {
        glfw::WindowEvent::FramebufferSize(width, height) => {
            // make sure the viewport matches the new window dimensions; note that width and
            // height will be significantly larger than specified on retina displays.
            unsafe { gl::Viewport(0, 0, width, height) }
        }
        glfw::WindowEvent::CursorPos(xpos, ypos) => {
            if skip_input { return; }
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

            camera.process_mouse_movement(xoffset, yoffset, true);
        }
        glfw::WindowEvent::Scroll(_xoffset, yoffset) => {
            if skip_input { return; }
            camera.process_mouse_scroll(yoffset as f32);
        }
        _ => {}
    }
}

/// Input processing function as introduced in 1.7.4 (Camera Class) and used in
/// most later tutorials
pub fn process_input(window: &mut glfw::Window, delta_time: f32, camera: &mut Camera) {
    if window.get_key(Key::Escape) == Action::Press {
        window.set_should_close(true)
    }

    if window.get_key(Key::W) == Action::Press {
        camera.process_keyboard(Camera_Movement::FORWARD, delta_time);
    }
    if window.get_key(Key::S) == Action::Press {
        camera.process_keyboard(Camera_Movement::BACKWARD, delta_time);
    }
    if window.get_key(Key::A) == Action::Press {
        camera.process_keyboard(Camera_Movement::LEFT, delta_time);
    }
    if window.get_key(Key::D) == Action::Press {
        camera.process_keyboard(Camera_Movement::RIGHT, delta_time);
    }
    camera.enable_mouse_movement(window.get_key(Key::LeftControl) != Action::Press);
}
