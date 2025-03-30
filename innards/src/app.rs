use wasm_bindgen::prelude::wasm_bindgen;

use crate::webgl_manager::WebglManager;

#[wasm_bindgen]
pub struct App {
    webgl_manager: WebglManager,
}

#[wasm_bindgen]
impl App {
    pub fn new(webgl_manager: WebglManager) -> App {
        App { webgl_manager }
    }

    pub fn draw(
        &self,
        u_time: f32,
        u_num_array_elements: usize,
        spacing: f32,
        u_frequency: f32,
        u_phase: f32,
        zoom: f32,
    ) {
        self.webgl_manager.draw_frame(
            u_time,
            u_num_array_elements,
            spacing,
            u_frequency,
            u_phase,
            zoom,
        );
    }
}
