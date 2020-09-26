use crate::gaia::imgui_helper::*;
use crate::gaia::shader::Shader;
use cgmath::{perspective, vec3, Deg, Matrix4, Point3, Vector3};
use imgui_glfw_rs::imgui;
use imgui_inspect_derive::Inspect;

#[derive(Inspect, Clone, Copy, Debug)]
pub struct PointLight {
    #[inspect(proxy_type = "CgmathVec3f32")]
    pub pos: Vector3<f32>,
    #[inspect(proxy_type = "CgmathVec3f32")]
    pub ambient: Vector3<f32>,
    #[inspect(proxy_type = "CgmathVec3f32")]
    pub diffuse: Vector3<f32>,
    #[inspect(proxy_type = "CgmathVec3f32")]
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

#[derive(Inspect, Clone, Copy, Debug)]
pub struct DirectionalLight {
    #[inspect(proxy_type = "CgmathVec3f32")]
    pub direction: Vector3<f32>,
    #[inspect(proxy_type = "CgmathVec3f32")]
    pub ambient: Vector3<f32>,
    #[inspect(proxy_type = "CgmathVec3f32")]
    pub diffuse: Vector3<f32>,
    #[inspect(proxy_type = "CgmathVec3f32")]
    pub specular: Vector3<f32>,
}

impl Default for DirectionalLight {
    fn default() -> Self {
        DirectionalLight {
            direction: vec3(-0.3, -1.0, -0.3),
            ambient: vec3(0.14, 0.14, 0.14),
            diffuse: vec3(0.4, 0.4, 0.4),
            specular: vec3(0.5, 0.5, 0.5),
        }
    }
}

impl DirectionalLight {
    pub unsafe fn shader_update(&self, shader: &Shader) {
        shader.set_vec3(
            c_str!("dirLight.direction"),
            self.direction.x,
            self.direction.y,
            self.direction.z,
        );
        shader.set_vec3(
            c_str!("dirLight.ambient"),
            self.ambient.x,
            self.ambient.y,
            self.ambient.z,
        );
        shader.set_vec3(
            c_str!("dirLight.diffuse"),
            self.diffuse.x,
            self.diffuse.y,
            self.diffuse.z,
        );
        shader.set_vec3(
            c_str!("dirLight.specular"),
            self.specular.x,
            self.specular.y,
            self.specular.z,
        );
    }
}

pub struct LightingSystem {
    pub shader: Shader,
    pub point_lights: [PointLight; 4],
    pub directional_light: DirectionalLight,
}

impl LightingSystem {
    pub unsafe fn prepare_for_draw(
        &self,
        projection: &Matrix4<f32>,
        view: &Matrix4<f32>,
        view_pos: &Vector3<f32>,
    ) {
        self.shader.use_program();

        self.shader.set_mat4(c_str!("projection"), projection);
        self.shader.set_mat4(c_str!("view"), view);
        self.shader.set_vector3(c_str!("viewPos"), view_pos);
        self.shader.setFloat(c_str!("material.shininess"), 32.0);

        self.directional_light.shader_update(&self.shader);

        for (i, v) in self.point_lights.iter().enumerate() {
            v.shader_update(i, &self.shader);
        }
    }
}

impl Default for LightingSystem {
    fn default() -> Self {
        LightingSystem {
            point_lights: [
                PointLight {
                    pos: vec3(0.7, 5.0, 2.0),
                    ..PointLight::default()
                },
                PointLight {
                    pos: vec3(2.3, 3.3, -4.0),
                    ..PointLight::default()
                },
                PointLight {
                    pos: vec3(-4.0, 4.0, -12.0),
                    ..PointLight::default()
                },
                PointLight {
                    pos: vec3(0.0, 2.0, -3.0),
                    ..PointLight::default()
                },
            ],
            directional_light: DirectionalLight::default(),
            shader: Shader::from_file(
                "resources/shaders/multiple_lights.vs",
                "resources/shaders/multiple_lights.fs",
            ),
        }
    }
}
