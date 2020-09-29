use crate::gaia::shader::*;
use gl::types::*;
use log::{info, warn};
use std::mem;
use std::os::raw::c_void;
use std::ptr;

#[derive(Debug)]
pub struct FramebufferSystem {
    pub shader: Shader,
    pub framebuffer: u32,
    pub texture_color_buffer: u32,
    vao: u32,
    vbo: u32,
}

impl Drop for FramebufferSystem {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteFramebuffers(1, &self.framebuffer);
        }
    }
}

impl FramebufferSystem {
    pub unsafe fn clear(&mut self) {
        // bind to framebuffer and draw scene as we normally would to color texture
        gl::BindFramebuffer(gl::FRAMEBUFFER, self.framebuffer);
        gl::Enable(gl::DEPTH_TEST); // enable depth testing (is disabled for rendering screen-space quad)

        // make sure we clear the framebuffer's content
        gl::ClearColor(0.1, 0.1, 0.1, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }

    pub unsafe fn draw(&mut self) {
        // now bind back to default framebuffer and draw a quad plane with the attached framebuffer color texture
        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        gl::Disable(gl::DEPTH_TEST); // disable depth test so screen-space quad isn't discarded due to depth test.
                                     // clear all relevant buffers
        gl::ClearColor(1.0, 0.0, 1.0, 1.0); // set clear color to white (not really necessery actually, since we won't be able to see behind the quad anyways)
        gl::Clear(gl::COLOR_BUFFER_BIT);

        self.shader.use_program();
        gl::BindVertexArray(self.vao);
        gl::BindTexture(gl::TEXTURE_2D, self.texture_color_buffer); // use the color attachment texture as the texture of the quad plane
        gl::DrawArrays(gl::TRIANGLES, 0, 6);
    }
    pub unsafe fn generate(scr_width: i32, scr_height: i32) -> Self {
        info!(
            "Generating new framebuffer with dimensions {}x{}",
            scr_width, scr_height
        );
        let shader = Shader::from_file(
            "resources/shaders/framebuffers_screen.vs",
            "resources/shaders/framebuffers_screen.fs",
        );

        let quad_vert: [f32; 24] = [
            // vertex attributes for a quad that fills the entire screen in Normalized Device Coordinates.
            // positions // texCoords
            -1.0, 1.0, 0.0, 1.0, -1.0, -1.0, 0.0, 0.0, 1.0, -1.0, 1.0, 0.0, -1.0, 1.0, 0.0, 1.0,
            1.0, -1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0,
        ];

        // screen quad VAO
        let (mut quad_vao, mut quad_vbo) = (0, 0);
        gl::GenVertexArrays(1, &mut quad_vao);
        gl::GenBuffers(1, &mut quad_vbo);
        gl::BindVertexArray(quad_vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, quad_vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (quad_vert.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            &quad_vert[0] as *const f32 as *const c_void,
            gl::STATIC_DRAW,
        );
        gl::EnableVertexAttribArray(0);
        let stride = 4 * mem::size_of::<GLfloat>() as GLsizei;
        gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, stride, ptr::null());
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            stride,
            (2 * mem::size_of::<GLfloat>()) as *const c_void,
        );
        shader.use_program();
        shader.setInt(c_str!("screenTexture"), 0);
        shader.setFloat(c_str!("screen_width"), scr_width as f32);
        shader.setFloat(c_str!("screen_height"), scr_height as f32);

        // framebuffer configuration
        // -------------------------
        let mut framebuffer = 0;
        gl::GenFramebuffers(1, &mut framebuffer);
        gl::BindFramebuffer(gl::FRAMEBUFFER, framebuffer);
        // create a color attachment texture
        let mut texture_color_buffer = 0;
        gl::GenTextures(1, &mut texture_color_buffer);
        gl::BindTexture(gl::TEXTURE_2D, texture_color_buffer);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as i32,
            scr_width,
            scr_height,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            ptr::null(),
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        gl::FramebufferTexture2D(
            gl::FRAMEBUFFER,
            gl::COLOR_ATTACHMENT0,
            gl::TEXTURE_2D,
            texture_color_buffer,
            0,
        );
        // create a renderbuffer object for depth and stencil attachment (we won't be sampling these)
        let mut rbo = 0;
        gl::GenRenderbuffers(1, &mut rbo);
        gl::BindRenderbuffer(gl::RENDERBUFFER, rbo);
        gl::RenderbufferStorage(
            gl::RENDERBUFFER,
            gl::DEPTH24_STENCIL8,
            scr_width,
            scr_height,
        ); // use a single renderbuffer object for both a depth AND stencil buffer.
        gl::FramebufferRenderbuffer(
            gl::FRAMEBUFFER,
            gl::DEPTH_STENCIL_ATTACHMENT,
            gl::RENDERBUFFER,
            rbo,
        ); // now actually attach it
           // now that we actually created the framebuffer and added all attachments we want to check if it is actually complete now
        if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
            warn!("ERROR::FRAMEBUFFER:: Framebuffer is not complete!");
        }
        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

        info!("New framebuffer generated");

        FramebufferSystem {
            texture_color_buffer: texture_color_buffer,
            shader: shader,
            vao: quad_vao,
            vbo: quad_vbo,
            framebuffer: framebuffer,
        }
    }
}
