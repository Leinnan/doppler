use cgmath::{Point3, Vector3};
use imgui_glfw_rs::imgui;
use imgui_inspect::InspectArgsDefault;
use imgui_inspect::InspectRenderDefault;

pub struct CgmathPoint3f32;
pub struct CgmathVec3f32;
impl InspectRenderDefault<Vector3<f32>> for CgmathVec3f32 {
    fn render(
        data: &[&Vector3<f32>],
        label: &'static str,
        ui: &imgui::Ui,
        _args: &InspectArgsDefault,
    ) {
        ui.text(label);
        for el in data.iter() {
            let text = format!("X: {:.3}, Y: {:.3}, Z: {:.3}", el.x, el.y, el.z);
            ui.text(&text);
        }
    }

    fn render_mut(
        data: &mut [&mut Vector3<f32>],
        label: &'static str,
        ui: &imgui::Ui,
        _args: &InspectArgsDefault,
    ) -> bool {
        use imgui::*;
        let label_im = im_str!("##x,y,z{}", label);
        ui.text(label);
        let mut change = false;
        for el in data.iter_mut() {
            let mut array: [f32; 3] = [el.x, el.y, el.z];
            change |= ui.drag_float3(&label_im, &mut array).build();
            el.x = array[0];
            el.y = array[1];
            el.z = array[2];
        }
        change
    }
}

impl InspectRenderDefault<Point3<f32>> for CgmathPoint3f32 {
    fn render(
        data: &[&Point3<f32>],
        label: &'static str,
        ui: &imgui::Ui,
        _args: &InspectArgsDefault,
    ) {
        ui.text(label);
        for el in data.iter() {
            let text = format!("X: {:.3}, Y: {:.3}, Z: {:.3}", el.x, el.y, el.z);
            ui.text(&text);
        }
    }

    fn render_mut(
        data: &mut [&mut Point3<f32>],
        label: &'static str,
        ui: &imgui::Ui,
        _args: &InspectArgsDefault,
    ) -> bool {
        use imgui::*;
        let label_im = im_str!("##x,y,z{}", label);
        ui.text(label);
        let mut change = false;
        for el in data.iter_mut() {
            let mut array: [f32; 3] = [el.x, el.y, el.z];
            change |= ui.drag_float3(&label_im, &mut array).build();
            el.x = array[0];
            el.y = array[1];
            el.z = array[2];
        }
        change
    }
}
