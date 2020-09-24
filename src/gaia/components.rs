use crate::gaia::*;
use crate::gaia::macros;
use cgmath::{perspective, vec3, Deg, Matrix4, Point3, Rad, Vector3};
use crate::gaia::model::Model;
use crate::gaia::shader::Shader;

pub struct Transform {
    pub position: Vector3<f32>,
    pub scale: Vector3<f32>,
    pub rotation: Vector3<f32>,
}

impl Default for Transform {
    fn default() -> Transform {
        Transform {
            position: vec3(0.0,0.0,0.0),
            scale: vec3(1.0,1.0,1.0),
            rotation: vec3(0.0,0.0,0.0),
        }
    }
}

impl Transform {
    pub fn get_matrix(&self) -> Matrix4<f32> {
        let mut m = Matrix4::<f32>::from_translation(self.position);
        m = m * Matrix4::from_nonuniform_scale(self.scale.x,self.scale.y,self.scale.z);

        m
    }
}

pub struct ModelComponent {
    pub model: Model,
    pub transform: Transform,
}

impl ModelComponent {
    pub unsafe fn draw(&mut self, shader: &Shader ) {
        let matrix = self.transform.get_matrix();

        shader.setMat4(c_str!("model"), &matrix);
        self.model.Draw(&shader);
    }
}