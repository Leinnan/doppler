use imgui_glfw_rs::imgui;
use imgui_inspect_derive::Inspect;

#[derive(Inspect)]
pub struct BgInfo {
    #[inspect_slider(min_value = 0.0, max_value = 1.0)]
    pub r: f32,
    #[inspect_slider(min_value = 0.0, max_value = 1.0)]
    pub g: f32,
    #[inspect_slider(min_value = 0.0, max_value = 1.0)]
    pub b: f32,
}

impl Default for BgInfo {
    fn default() -> Self {
        BgInfo {
            r: 0.1,
            g: 0.2,
            b: 0.4,
        }
    }
}
