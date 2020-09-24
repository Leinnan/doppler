use crate::gaia::model::Model;
use crate::gaia::shader::Shader;
use cgmath::{vec3, Matrix4, Vector3, Rad};
use imgui_glfw_rs::imgui;
use imgui_inspect::InspectArgsDefault;
use imgui_inspect::InspectRenderDefault;
use imgui_inspect_derive::Inspect;

struct CgmathVec3f32;
impl InspectRenderDefault<Vector3<f32>> for CgmathVec3f32 {
    fn render(
        data: &[&Vector3<f32>],
        label: &'static str,
        ui: &imgui::Ui,
        _args: &InspectArgsDefault,
    ) {
        ui.text(label);
        ui.text(format!("{:?}", data));
    }

    fn render_mut(
        data: &mut [&mut Vector3<f32>],
        label: &'static str,
        ui: &imgui::Ui,
        _args: &InspectArgsDefault,
    ) -> bool {
        use imgui::*;
        let id_x = im_str!("x##{}", label);
        let id_y = im_str!("y##{}", label);
        let id_z = im_str!("z##{}", label);
        ui.text(label);
        let mut change = false;
        for el in data.iter_mut() {
            change |= ui
                .input_float(&id_x, &mut el.x)
                .step(0.01)
                .step_fast(1.0)
                .build();
            change |= ui
                .input_float(&id_y, &mut el.y)
                .step(0.01)
                .step_fast(1.0)
                .build();
            change |= ui
                .input_float(&id_z, &mut el.z)
                .step(0.01)
                .step_fast(1.0)
                .build();
        }
        change
    }
}

#[derive(Inspect, Clone, Copy, Debug)]
pub struct Transform {
    #[inspect(proxy_type = "CgmathVec3f32")]
    pub scale: Vector3<f32>,
    #[inspect(proxy_type = "CgmathVec3f32")]
    pub position: Vector3<f32>,
    #[inspect(proxy_type = "CgmathVec3f32")]
    pub rotation: Vector3<f32>,
}

impl Default for Transform {
    fn default() -> Transform {
        Transform {
            position: vec3(0.0, 0.0, 0.0),
            scale: vec3(1.0, 1.0, 1.0),
            rotation: vec3(0.0, 0.0, 0.0),
        }
    }
}

impl Transform {
    pub fn get_matrix(&self) -> Matrix4<f32> {
        let mut m = Matrix4::<f32>::from_translation(self.position);
        m = m * Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z);
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
    pub unsafe fn draw(&mut self, shader: &Shader) {
        let matrix = self.transform.get_matrix();

        shader.set_mat4(c_str!("model"), &matrix);
        self.model.Draw(&shader);
    }
}
