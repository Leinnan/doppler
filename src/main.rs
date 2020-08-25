extern crate gl;
extern crate glutin;
extern crate image;

use self::camera::*;
use self::gl::types::*;
// use self::glfw::{Action, Context, Key};

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use cgmath::prelude::*;
use cgmath::{perspective, vec3, Deg, Matrix4, Point3, Rad, Vector3};
use human_panic::setup_panic;
use image::GenericImageView;
use std::ffi::CString;
use std::mem;
use std::os::raw::c_void;
use std::ptr;
use std::sync::mpsc::Receiver;
mod camera;
mod consts;
mod engine;
mod macros;
mod shader;
mod model;
mod mesh;

const CUBES_POS: [Vector3<f32>; 10] = [
    vec3(0.0, 0.0, 0.0),
    vec3(2.0, 5.0, -15.0),
    vec3(-1.5, -2.2, -2.5),
    vec3(-3.8, -2.0, -12.3),
    vec3(2.4, -0.4, -3.5),
    vec3(-1.7, 3.0, -7.5),
    vec3(1.3, -2.0, -2.5),
    vec3(1.5, 2.0, -2.5),
    vec3(1.5, 0.2, -1.5),
    vec3(-1.3, 1.0, -1.5),
];

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
    let mut events_loop = EventLoop::new();
    let window = WindowBuilder::new().with_title("Doppler");
    let gl_window = glutin::ContextBuilder::new()
        .build_windowed(window, &events_loop)
        .unwrap();

    // window.make_current();
    // window.set_framebuffer_size_polling(true);
    // window.set_cursor_pos_polling(true);
    // window.set_scroll_polling(true);
    // window.set_cursor_mode(glfw::CursorMode::Disabled);

    // gl: load all OpenGL function pointers
    // ---------------------------------------

    // It is essential to make the context current before calling `gl::load_with`.
    let gl_window = unsafe { gl_window.make_current().unwrap() };

    // Load the OpenGL function pointers
    // TODO: `as *const _` will not be needed once glutin is updated to the latest gl version
    gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);

    
    let (our_shader, our_model) = unsafe {
        // configure global opengl state
        // -----------------------------
        gl::Enable(gl::DEPTH_TEST);

        // build and compile shaders
        // -------------------------
        let our_shader = shader::Shader::from_file(
            "resources/shaders/model_loading.vs",
            "resources/shaders/model_loading.fs");

        // load models
        // -----------
        let our_model = model::Model::new("resources/objects/planet/planet.obj");

        // draw in wireframe
        // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

        (our_shader, our_model)
    };



    // render loop
    // -----------
    let (r, g, b) = (0.188, 0.22, 0.235);
    let delta_time = 0.3;
    events_loop.run(move |event, _, control_flow| {
        println!("{:?}", event);
        *control_flow = ControlFlow::Wait;

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => {
                    gl_window.resize(physical_size)
                }
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    
                    let key = if input.virtual_keycode.is_none() {
                        glutin::event::VirtualKeyCode::Escape
                    } else {
                        input.virtual_keycode.unwrap()
                    };
                    match key {
                        glutin::event::VirtualKeyCode::W => {
                            camera.process_keyboard(Camera_Movement::FORWARD, delta_time);
                        }
                        glutin::event::VirtualKeyCode::S => {
                            camera.process_keyboard(Camera_Movement::BACKWARD, delta_time);
                        }
                        glutin::event::VirtualKeyCode::A => {
                            camera.process_keyboard(Camera_Movement::LEFT, delta_time);
                        }
                        glutin::event::VirtualKeyCode::D => {
                            camera.process_keyboard(Camera_Movement::RIGHT, delta_time);
                        }
                        _ => (),
                    }
                   println!("{:?}", input);
                   println!("{:?}", input);
                   println!("{:?}", input);
                   println!("{:?}", input);
                }
                _ => (),
            },
            Event::RedrawRequested(_) => {
                unsafe{gl::ClearColor(r, g, b, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                        // don't forget to enable shader before setting uniforms
            our_shader.useProgram();

            // view/projection transformations
            let projection: Matrix4<f32> = perspective(Deg(camera.Zoom), consts::SCR_WIDTH as f32 / consts::SCR_HEIGHT as f32, 0.1, 100.0);
            let view = camera.get_view_matrix();
            our_shader.setMat4(c_str!("projection"), &projection);
            our_shader.setMat4(c_str!("view"), &view);

            // render the loaded model
            let mut model = Matrix4::<f32>::from_translation(vec3(0.0, -1.75, 0.0)); // translate it down so it's at the center of the scene
            model = model * Matrix4::from_scale(0.2);  // it's a bit too big for our scene, so scale it down
            our_shader.setMat4(c_str!("model"), &model);
            our_model.Draw(&our_shader);
                }
                gl_window.swap_buffers().unwrap();
            }
            _ => (),
        }
    });

    // while !window.should_close() {
    //     // per-frame time logic
    //     // --------------------
    //     let cur_frame = glfw.get_time() as f32;
    //     delta_time = cur_frame - last_frame;
    //     last_frame = cur_frame;

    //     // events
    //     // -----
    //     process_events(
    //         &events,
    //         &mut first_mouse,
    //         &mut last_x,
    //         &mut last_y,
    //         &mut camera,
    //     );

    //     // input
    //     // -----
    //     process_input(&mut window, delta_time, &mut camera);

    //     // render
    //     // ------


    //     // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
    //     // -------------------------------------------------------------------------------
    //     window.swap_buffers();
    //     glfw.poll_events();
    // }
}

// pub fn process_events(
//     events: &Receiver<(f64, glfw::WindowEvent)>,
//     first_mouse: &mut bool,
//     last_x: &mut f32,
//     last_y: &mut f32,
//     camera: &mut Camera,
// ) {
//     for (_, event) in glfw::flush_messages(events) {
//         match event {
//             glfw::WindowEvent::FramebufferSize(width, height) => {
//                 // make sure the viewport matches the new window dimensions; note that width and
//                 // height will be significantly larger than specified on retina displays.
//                 unsafe { gl::Viewport(0, 0, width, height) }
//             }
//             glfw::WindowEvent::CursorPos(xpos, ypos) => {
//                 let (xpos, ypos) = (xpos as f32, ypos as f32);
//                 if *first_mouse {
//                     *last_x = xpos;
//                     *last_y = ypos;
//                     *first_mouse = false;
//                 }

//                 let xoffset = xpos - *last_x;
//                 let yoffset = *last_y - ypos; // reversed since y-coordinates go from bottom to top

//                 *last_x = xpos;
//                 *last_y = ypos;

//                 camera.process_mouse_movement(xoffset, yoffset, true);
//             }
//             glfw::WindowEvent::Scroll(_xoffset, yoffset) => {
//                 camera.process_mouse_scroll(yoffset as f32);
//             }
//             _ => {}
//         }
//     }
// }

// /// Input processing function as introduced in 1.7.4 (Camera Class) and used in
// /// most later tutorials
// pub fn process_input(window: &mut glfw::Window, delta_time: f32, camera: &mut Camera) {
//     if window.get_key(Key::Escape) == Action::Press {
//         window.set_should_close(true)
//     }

//     if window.get_key(Key::W) == Action::Press {
//         camera.process_keyboard(Camera_Movement::FORWARD, delta_time);
//     }
//     if window.get_key(Key::S) == Action::Press {
//         camera.process_keyboard(Camera_Movement::BACKWARD, delta_time);
//     }
//     if window.get_key(Key::A) == Action::Press {
//         camera.process_keyboard(Camera_Movement::LEFT, delta_time);
//     }
//     if window.get_key(Key::D) == Action::Press {
//         camera.process_keyboard(Camera_Movement::RIGHT, delta_time);
//     }
// }
