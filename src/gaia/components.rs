use crate::gaia::imgui_helper::*;
use crate::gaia::model::Model;
use crate::gaia::shader::Shader;
use cgmath::{vec3, Matrix4, Rad, Vector3};
use imgui_glfw_rs::imgui;
use imgui_inspect_derive::Inspect;

#[derive(Inspect, Clone, Copy, Debug)]
pub struct Transform {
    #[inspect(proxy_type = "CgmathVec3f32")]
    pub position: Vector3<f32>,
    #[inspect(proxy_type = "CgmathVec3f32")]
    pub rotation: Vector3<f32>,
    pub scale: f32,
}

impl Default for Transform {
    fn default() -> Transform {
        Transform {
            position: vec3(0.0, 0.0, 0.0),
            rotation: vec3(0.0, 0.0, 0.0),
            scale: 1.0,
        }
    }
}

impl Transform {
    pub fn get_matrix(&self) -> Matrix4<f32> {
        let mut m = Matrix4::<f32>::from_translation(self.position);
        m = m * Matrix4::from_scale(self.scale);
        m = m * Matrix4::<f32>::from_angle_x(Rad(self.rotation.x.to_radians()));
        m = m * Matrix4::<f32>::from_angle_y(Rad(self.rotation.y.to_radians()));
        m = m * Matrix4::<f32>::from_angle_z(Rad(self.rotation.z.to_radians()));

        m
    }
}

pub struct ModelComponent {
    pub model: Model,
    pub transform: Transform,
}

impl ModelComponent {
    pub unsafe fn draw(&self, shader: &Shader) {
        let matrix = self.transform.get_matrix();

        shader.set_mat4(c_str!("model"), &matrix);
        self.model.Draw(&shader);
    }
}
