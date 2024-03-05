use crate::shader::*;
use crate::utils::*;
use cgmath::Matrix4;
use gl::types::*;
use std::mem;
use std::os::raw::c_void;
use std::ptr;

pub struct Sky {
    shader: Shader,
    texture_id: u32,
    vao: u32,
    vbo: u32,
}

impl Drop for Sky {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
        }
    }
}

impl Sky {
    pub unsafe fn new() -> Sky {
        let shader =
            Shader::from_file("resources/shaders/skybox.vs", "resources/shaders/skybox.fs");

        let skybox_vertices: [f32; 108] = [
            // positions
            -1.0, 1.0, -1.0, -1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0,
            -1.0, 1.0, -1.0, -1.0, -1.0, 1.0, -1.0, -1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0,
            -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
            1.0, 1.0, 1.0, 1.0, -1.0, 1.0, -1.0, -1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0,
            1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0,
            1.0, 1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, -1.0, -1.0, -1.0, -1.0, -1.0,
            -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0,
        ];

        // skybox VAO
        let (mut skybox_vao, mut skybox_vbo) = (0, 0);
        gl::GenVertexArrays(1, &mut skybox_vao);
        gl::GenBuffers(1, &mut skybox_vbo);
        gl::BindVertexArray(skybox_vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, skybox_vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (skybox_vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            &skybox_vertices[0] as *const f32 as *const c_void,
            gl::STATIC_DRAW,
        );
        gl::EnableVertexAttribArray(0);
        let stride = 3 * mem::size_of::<GLfloat>() as GLsizei;
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());

        let faces = [
            "resources/objects/skybox/right.jpg",
            "resources/objects/skybox/left.jpg",
            "resources/objects/skybox/top.jpg",
            "resources/objects/skybox/bottom.jpg",
            "resources/objects/skybox/back.jpg",
            "resources/objects/skybox/front.jpg",
        ];
        let cubemap_texture = load_cubemap(&faces);
        shader.use_program();
        shader.setInt(c_str!("skybox"), 0);

        Sky {
            shader,
            texture_id: cubemap_texture,
            vao: skybox_vao,
            vbo: skybox_vbo,
        }
    }

    pub unsafe fn draw(&mut self, mut view: Matrix4<f32>, projection: Matrix4<f32>) {
        gl::DepthFunc(gl::LEQUAL); // change depth function so depth test passes when values are equal to depth buffer's content
        self.shader.use_program();
        // remove translation from the view matrix
        view.w[0] = 0.0;
        view.w[1] = 0.0;
        view.w[2] = 0.0;
        self.shader.set_mat4(c_str!("view"), &view);
        self.shader.set_mat4(c_str!("projection"), &projection);
        // skybox cube
        gl::BindVertexArray(self.vao);
        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_CUBE_MAP, self.texture_id);
        gl::DrawArrays(gl::TRIANGLES, 0, 36);
        gl::BindVertexArray(0);
        gl::DepthFunc(gl::LESS); // set depth function back to default
    }
}
