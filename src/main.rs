extern crate gl;
extern crate glfw;
extern crate image;

use self::glfw::{Action, Context, Key};
use self::gl::types::*;
use std::ffi::CString;
use std::mem;
use std::os::raw::c_void;
use std::ptr;
use std::sync::mpsc::Receiver;
use cgmath::prelude::*;
use cgmath::{perspective, vec3, Deg, Matrix4, Rad};
use image::GenericImageView;

mod consts;
mod macros;
mod shader;

pub fn main() {
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
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    // gl: load all OpenGL function pointers
    // ---------------------------------------
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let (shader_object, vbo, vao, texture) = unsafe {
        gl::Enable(gl::DEPTH_TEST);
        let v_shader = CString::new(consts::VERTEX_SHADER_SRC.as_bytes()).unwrap();
        let f_shader = CString::new(consts::FRAGMENT_SHADER_SRC.as_bytes()).unwrap();
        let shader = crate::shader::Shader::new(v_shader, f_shader);
        // set up vertex data (and buffer(s)) and configure vertex attributes
        // ------------------------------------------------------------------
        // HINT: type annotation is crucial since default for float literals is f64
        let vertices: [f32; 180] = [
            -0.5, -0.5, -0.5,  0.0, 0.0,
             0.5, -0.5, -0.5,  1.0, 0.0,
             0.5,  0.5, -0.5,  1.0, 1.0,
             0.5,  0.5, -0.5,  1.0, 1.0,
            -0.5,  0.5, -0.5,  0.0, 1.0,
            -0.5, -0.5, -0.5,  0.0, 0.0,

            -0.5, -0.5,  0.5,  0.0, 0.0,
             0.5, -0.5,  0.5,  1.0, 0.0,
             0.5,  0.5,  0.5,  1.0, 1.0,
             0.5,  0.5,  0.5,  1.0, 1.0,
            -0.5,  0.5,  0.5,  0.0, 1.0,
            -0.5, -0.5,  0.5,  0.0, 0.0,

            -0.5,  0.5,  0.5,  1.0, 0.0,
            -0.5,  0.5, -0.5,  1.0, 1.0,
            -0.5, -0.5, -0.5,  0.0, 1.0,
            -0.5, -0.5, -0.5,  0.0, 1.0,
            -0.5, -0.5,  0.5,  0.0, 0.0,
            -0.5,  0.5,  0.5,  1.0, 0.0,

             0.5,  0.5,  0.5,  1.0, 0.0,
             0.5,  0.5, -0.5,  1.0, 1.0,
             0.5, -0.5, -0.5,  0.0, 1.0,
             0.5, -0.5, -0.5,  0.0, 1.0,
             0.5, -0.5,  0.5,  0.0, 0.0,
             0.5,  0.5,  0.5,  1.0, 0.0,

            -0.5, -0.5, -0.5,  0.0, 1.0,
             0.5, -0.5, -0.5,  1.0, 1.0,
             0.5, -0.5,  0.5,  1.0, 0.0,
             0.5, -0.5,  0.5,  1.0, 0.0,
            -0.5, -0.5,  0.5,  0.0, 0.0,
            -0.5, -0.5, -0.5,  0.0, 1.0,

            -0.5,  0.5, -0.5,  0.0, 1.0,
             0.5,  0.5, -0.5,  1.0, 1.0,
             0.5,  0.5,  0.5,  1.0, 0.0,
             0.5,  0.5,  0.5,  1.0, 0.0,
            -0.5,  0.5,  0.5,  0.0, 0.0,
            -0.5,  0.5, -0.5,  0.0, 1.0
       ];
       let (mut vbo, mut vao) = (0, 0);
       gl::GenVertexArrays(1, &mut vao);
       gl::GenBuffers(1, &mut vbo);

       gl::BindVertexArray(vao);

       gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
       gl::BufferData(gl::ARRAY_BUFFER,
                      (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                      &vertices[0] as *const f32 as *const c_void,
                      gl::STATIC_DRAW);

       let stride = 5 * mem::size_of::<GLfloat>() as GLsizei;
       // position attribute
       gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
       gl::EnableVertexAttribArray(0);
       // texture coord attribute
       gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<GLfloat>()) as *const c_void);
       gl::EnableVertexAttribArray(1);


        // uncomment this call to draw in wireframe polygons.
        // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

        // -------------------------
        let mut texture = 0;
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture); // all upcoming GL_TEXTURE_2D operations now have effect on this texture object
                                                  // set the texture wrapping parameters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32); // set texture wrapping to gl::REPEAT (default wrapping method)
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        // set texture filtering parameters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        // load image, create texture and generate mipmaps
        let img = image::open("resources/garlic_dog_space.jpg").unwrap().flipv();
        let data = img.raw_pixels();

        // let data = flipped
        let dimensions = (img.width(), img.height());
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as i32,
            dimensions.0 as i32,
            dimensions.1 as i32,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            &data[0] as *const u8 as *const c_void,
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);
        (shader, vbo, vao, texture)
    };

    // render loop
    // -----------
    let color_r = 0.188;
    let color_g = 0.22;
    let color_b = 0.235;
    let mut view_modifier : f32 = 0.5;
    let mut view_multiplier = 0.01;
    while !window.should_close() {
        view_modifier = view_modifier + view_multiplier;
        if view_modifier > 2.0 || view_modifier < 0.0 {
            view_multiplier = -view_multiplier;
        }
        // events
        // -----
        process_events(&mut window, &events);

        // render
        // ------
        unsafe {
            gl::ClearColor(color_r, color_g, color_b, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            gl::BindTexture(gl::TEXTURE_2D, texture);
            shader_object.useProgram();

            // create transformations
            // NOTE: cgmath requires axis vectors to be normalized!
            let model: Matrix4<f32> = Matrix4::from_axis_angle(vec3(0.5, 1.0, 0.0).normalize(),
                                                               Rad(glfw.get_time() as f32));
            let view: Matrix4<f32> = Matrix4::from_translation(vec3(0., 0., -5.0+view_modifier));
            let projection: Matrix4<f32> = perspective(Deg(45.0), consts::SCR_WIDTH as f32 / consts::SCR_HEIGHT as f32, 0.1, 100.0);
            // retrieve the matrix uniform locations
            let model_loc = gl::GetUniformLocation(shader_object.ID, c_str!("model").as_ptr());
            let view_loc = gl::GetUniformLocation(shader_object.ID, c_str!("view").as_ptr());
            // pass them to the shaders (3 different ways)
            gl::UniformMatrix4fv(model_loc, 1, gl::FALSE, model.as_ptr());
            gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, &view[0][0]);
            // note: currently we set the projection matrix each frame, but since the projection matrix rarely changes it's often best practice to set it outside the main loop only once.
            shader_object.setMat4(c_str!("projection"), &projection);

            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }

        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
        // -------------------------------------------------------------------------------
        window.swap_buffers();
        glfw.poll_events();
    }

    unsafe {
        gl::DeleteVertexArrays(1, &vao);
        gl::DeleteBuffers(1, &vbo);
    }
}

// NOTE: not the same version as in common.rs!
fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions; note that width and
                // height will be significantly larger than specified on retina displays.
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true)
            }
            glfw::WindowEvent::Key(_, _, Action::Press, _) => println!("Other key pressed"),
            _ => {}
        }
    }
}
