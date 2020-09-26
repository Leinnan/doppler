use cgmath::{vec3, Matrix4, Rad, Vector3};
use imgui_glfw_rs::imgui;
use imgui_inspect::InspectArgsDefault;
use imgui_inspect::InspectRenderDefault;

pub struct CgmathVec3f32;
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
