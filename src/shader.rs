#![allow(non_snake_case)]
use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::Read;
use std::ptr;
use std::str;

use gl;
use gl::types::*;

use crate::consts;
use cgmath::prelude::*;
use cgmath::{Matrix, Matrix4, Vector2, Vector3};

#[derive(Debug)]
pub struct Shader {
    pub id: u32,
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

impl Default for Shader {
    fn default() -> Self {
        let vShaderCode = CString::new(consts::VERTEX_SHADER_SRC.as_bytes()).unwrap();
        let fShaderCode = CString::new(consts::FRAGMENT_SHADER_SRC.as_bytes()).unwrap();

        Shader::new(vShaderCode, fShaderCode)
    }
}

/// NOTE: mixture of `shader_s.h` and `shader_m.h` (the latter just contains
/// a few more setters for uniforms)
#[allow(dead_code)]
impl Shader {
    pub fn new(vShaderCode: CString, fShaderCode: CString) -> Shader {
        let mut shader = Shader { id: 0 };

        // 2. compile shaders
        unsafe {
            // vertex shader
            let vertex = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vertex, 1, &vShaderCode.as_ptr(), ptr::null());
            gl::CompileShader(vertex);
            shader.check_compile_errors(vertex, "VERTEX");
            // fragment Shader
            let fragment = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fragment, 1, &fShaderCode.as_ptr(), ptr::null());
            gl::CompileShader(fragment);
            shader.check_compile_errors(fragment, "FRAGMENT");
            // shader Program
            let ID = gl::CreateProgram();
            gl::AttachShader(ID, vertex);
            gl::AttachShader(ID, fragment);
            gl::LinkProgram(ID);
            shader.check_compile_errors(ID, "PROGRAM");
            // delete the shaders as they're linked into our program now and no longer necessary
            gl::DeleteShader(vertex);
            gl::DeleteShader(fragment);
            shader.id = ID;
        }

        shader
    }

    pub fn from_file(vertexPath: &str, fragmentPath: &str) -> Shader {
        // 1. retrieve the vertex/fragment source code from filesystem
        let mut vShaderFile =
            File::open(vertexPath).unwrap_or_else(|_| panic!("Failed to open {}", vertexPath));
        let mut fShaderFile =
            File::open(fragmentPath).unwrap_or_else(|_| panic!("Failed to open {}", fragmentPath));
        let mut vertexCode = String::new();
        let mut fragmentCode = String::new();
        vShaderFile
            .read_to_string(&mut vertexCode)
            .expect("Failed to read vertex shader");
        fShaderFile
            .read_to_string(&mut fragmentCode)
            .expect("Failed to read fragment shader");

        let vShaderCode = CString::new(vertexCode.as_bytes()).unwrap();
        let fShaderCode = CString::new(fragmentCode.as_bytes()).unwrap();

        Shader::new(vShaderCode, fShaderCode)
    }

    /// activate the shader
    /// ------------------------------------------------------------------------
    pub unsafe fn use_program(&self) {
        gl::UseProgram(self.id)
    }

    /// utility uniform functions
    /// ------------------------------------------------------------------------
    pub unsafe fn setBool(&self, name: &CStr, value: bool) {
        gl::Uniform1i(gl::GetUniformLocation(self.id, name.as_ptr()), value as i32);
    }
    /// ------------------------------------------------------------------------
    pub unsafe fn setInt(&self, name: &CStr, value: i32) {
        gl::Uniform1i(gl::GetUniformLocation(self.id, name.as_ptr()), value);
    }
    /// ------------------------------------------------------------------------
    pub unsafe fn setFloat(&self, name: &CStr, value: f32) {
        gl::Uniform1f(gl::GetUniformLocation(self.id, name.as_ptr()), value);
    }
    /// ------------------------------------------------------------------------
    pub unsafe fn set_vector3(&self, name: &CStr, value: &Vector3<f32>) {
        gl::Uniform3fv(
            gl::GetUniformLocation(self.id, name.as_ptr()),
            1,
            value.as_ptr(),
        );
    }
    pub unsafe fn set_vector2(&self, name: &CStr, value: &Vector2<f32>) {
        gl::Uniform2fv(
            gl::GetUniformLocation(self.id, name.as_ptr()),
            1,
            value.as_ptr(),
        );
    }
    /// ------------------------------------------------------------------------
    pub unsafe fn set_vec3(&self, name: &CStr, x: f32, y: f32, z: f32) {
        gl::Uniform3f(gl::GetUniformLocation(self.id, name.as_ptr()), x, y, z);
    }
    /// ------------------------------------------------------------------------
    pub unsafe fn set_mat4(&self, name: &CStr, mat: &Matrix4<f32>) {
        gl::UniformMatrix4fv(
            gl::GetUniformLocation(self.id, name.as_ptr()),
            1,
            gl::FALSE,
            mat.as_ptr(),
        );
    }

    /// utility function for checking shader compilation/linking errors.
    /// ------------------------------------------------------------------------
    unsafe fn check_compile_errors(&self, shader: u32, type_: &str) {
        let mut is_success = gl::FALSE as GLint;
        let mut infoLog = Vec::with_capacity(1024);
        infoLog.set_len(1024 - 1); // subtract 1 to skip the trailing null character
        if type_ != "PROGRAM" {
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut is_success);
            if is_success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(
                    shader,
                    1024,
                    ptr::null_mut(),
                    infoLog.as_mut_ptr() as *mut GLchar,
                );
                println!(
                    "ERROR::SHADER_COMPILATION_ERROR of type: {}\n{}\n \
                          -- --------------------------------------------------- -- ",
                    type_,
                    str::from_utf8(&infoLog).unwrap()
                );
            }
        } else {
            gl::GetProgramiv(shader, gl::LINK_STATUS, &mut is_success);
            if is_success != gl::TRUE as GLint {
                gl::GetProgramInfoLog(
                    shader,
                    1024,
                    ptr::null_mut(),
                    infoLog.as_mut_ptr() as *mut GLchar,
                );
                println!(
                    "ERROR::PROGRAM_LINKING_ERROR of type: {}\n{}\n \
                          -- --------------------------------------------------- -- ",
                    type_,
                    str::from_utf8(&infoLog).unwrap()
                );
            }
        }
    }

    /// Only used in 4.9 Geometry shaders - ignore until then (shader.h in original C++)
    pub fn with_geometry_shader(vertexPath: &str, fragmentPath: &str, geometryPath: &str) -> Self {
        let mut shader = Shader { id: 0 };
        // 1. retrieve the vertex/fragment source code from filesystem
        let mut vShaderFile =
            File::open(vertexPath).unwrap_or_else(|_| panic!("Failed to open {}", vertexPath));
        let mut fShaderFile =
            File::open(fragmentPath).unwrap_or_else(|_| panic!("Failed to open {}", fragmentPath));
        let mut gShaderFile =
            File::open(geometryPath).unwrap_or_else(|_| panic!("Failed to open {}", geometryPath));
        let mut vertexCode = String::new();
        let mut fragmentCode = String::new();
        let mut geometryCode = String::new();
        vShaderFile
            .read_to_string(&mut vertexCode)
            .expect("Failed to read vertex shader");
        fShaderFile
            .read_to_string(&mut fragmentCode)
            .expect("Failed to read fragment shader");
        gShaderFile
            .read_to_string(&mut geometryCode)
            .expect("Failed to read geometry shader");

        let vShaderCode = CString::new(vertexCode.as_bytes()).unwrap();
        let fShaderCode = CString::new(fragmentCode.as_bytes()).unwrap();
        let gShaderCode = CString::new(geometryCode.as_bytes()).unwrap();

        // 2. compile shaders
        unsafe {
            // vertex shader
            let vertex = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vertex, 1, &vShaderCode.as_ptr(), ptr::null());
            gl::CompileShader(vertex);
            shader.check_compile_errors(vertex, "VERTEX");
            // fragment Shader
            let fragment = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fragment, 1, &fShaderCode.as_ptr(), ptr::null());
            gl::CompileShader(fragment);
            shader.check_compile_errors(fragment, "FRAGMENT");
            // geometry shader
            let geometry = gl::CreateShader(gl::GEOMETRY_SHADER);
            gl::ShaderSource(geometry, 1, &gShaderCode.as_ptr(), ptr::null());
            gl::CompileShader(geometry);
            shader.check_compile_errors(geometry, "GEOMETRY");

            // shader Program
            let ID = gl::CreateProgram();
            gl::AttachShader(ID, vertex);
            gl::AttachShader(ID, fragment);
            gl::AttachShader(ID, geometry);
            gl::LinkProgram(ID);
            shader.check_compile_errors(ID, "PROGRAM");
            // delete the shaders as they're linked into our program now and no longer necessary
            gl::DeleteShader(vertex);
            gl::DeleteShader(fragment);
            gl::DeleteShader(geometry);
            shader.id = ID;
        }

        shader
    }
}
