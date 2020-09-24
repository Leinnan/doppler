use imgui_inspect::InspectArgsDefault;
use imgui_inspect::InspectRenderDefault;
use cgmath::{perspective, vec3, Deg, Matrix4, Point3, Rad, Vector3};
use crate::gaia::model::Model;
use crate::gaia::shader::Shader;
use imgui_glfw_rs::imgui;
use imgui_inspect::*;
use imgui_inspect_derive::Inspect;
use imgui_inspect::InspectArgsStruct;

struct cgmathVec3f32;
impl InspectRenderDefault<Vector3<f32>> for cgmathVec3f32 {
    fn render(data: &[&Vector3<f32>], label: &'static str, ui: &imgui::Ui, args: &InspectArgsDefault) {
        ui.text(label);
        ui.text(format!("{:?}", data));
    }

    fn render_mut(data: &mut [&mut Vector3<f32>], label: &'static str, ui: &imgui::Ui, args: &InspectArgsDefault) -> bool {
        use imgui::*;
        ui.text(label);
        let mut change = false;
        for el in data.iter_mut() {
            change |= ui.input_float(im_str!("x"), &mut el.x)
                .step(0.01)
                .step_fast(1.0)
                .build();
                change |= ui.input_float(im_str!("y"), &mut el.y)
                    .step(0.01)
                    .step_fast(1.0)
                    .build();
                    change |= ui.input_float(im_str!("z"), &mut el.z)
                        .step(0.01)
                        .step_fast(1.0)
                        .build();
        }
        change
    }
}

#[derive(Inspect, Clone)]
pub struct Transform {
    #[inspect(proxy_type = "cgmathVec3f32")]
    pub position: Vector3<f32>,
    #[inspect(proxy_type = "cgmathVec3f32")]
    pub scale: Vector3<f32>,
    #[inspect(proxy_type = "cgmathVec3f32")]
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