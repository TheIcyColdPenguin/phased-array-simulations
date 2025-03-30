use app::App;
use console_error_panic_hook;
use std::panic;
use web_sys::WebGl2RenderingContext;
use webgl_manager::WebglManager;

use wasm_bindgen::prelude::*;

mod app;
mod webgl_manager;

#[wasm_bindgen]
pub fn initialise() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub fn create_app(
    context: WebGl2RenderingContext,
    width: f32,
    height: f32,
) -> Result<App, JsValue> {
    let manager = WebglManager::new(context, width, height)?;
    Ok(App::new(manager))
}
