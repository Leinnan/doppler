use crate::gaia::shader::Shader;
use cgmath::{perspective, vec3, Deg, Matrix4, Point3, Vector3};

pub struct PointLight {
    pub pos: Vector3<f32>,
    pub ambient: Vector3<f32>,
    pub diffuse: Vector3<f32>,
    pub specular: Vector3<f32>,
    pub constant: f32,
    pub linear: f32,
    pub quadratic: f32,
}

impl PointLight {
    pub unsafe fn shader_update(&self, i: usize, shader: &Shader) {
        let mut id = format!("pointLights[{}].position\0", i);
        shader.set_vector3(
            std::ffi::CStr::from_bytes_with_nul_unchecked(id.as_bytes()),
            &self.pos,
        );
        id = format!("pointLights[{}].ambient\0", i);
        shader.set_vec3(
            std::ffi::CStr::from_bytes_with_nul_unchecked(id.as_bytes()),
            self.ambient.x,
            self.ambient.y,
            self.ambient.z,
        );
        id = format!("pointLights[{}].diffuse\0", i);
        shader.set_vec3(
            std::ffi::CStr::from_bytes_with_nul_unchecked(id.as_bytes()),
            self.diffuse.x,
            self.diffuse.y,
            self.diffuse.z,
        );
        id = format!("pointLights[{}].specular\0", i);
        shader.set_vec3(
            std::ffi::CStr::from_bytes_with_nul_unchecked(id.as_bytes()),
            self.specular.x,
            self.specular.y,
            self.specular.z,
        );
        id = format!("pointLights[{}].constant\0", i);
        shader.setFloat(
            std::ffi::CStr::from_bytes_with_nul_unchecked(id.as_bytes()),
            self.constant,
        );
        id = format!("pointLights[{}].linear\0", i);
        shader.setFloat(
            std::ffi::CStr::from_bytes_with_nul_unchecked(id.as_bytes()),
            self.linear,
        );
        id = format!("pointLights[{}].quadratic\0", i);
        shader.setFloat(
            std::ffi::CStr::from_bytes_with_nul_unchecked(id.as_bytes()),
            self.quadratic,
        );
    }
}

impl Default for PointLight {
    fn default() -> Self {
        PointLight {
            pos: vec3(0.0, 0.0, 0.0),
            ambient: vec3(0.05, 0.05, 0.05),
            diffuse: vec3(0.8, 0.8, 0.8),
            specular: vec3(1.0, 1.0, 1.0),
            constant: 1.0,
            linear: 0.09,
            quadratic: 0.032,
        }
    }
}
