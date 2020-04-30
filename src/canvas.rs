use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
extern "C" {
    fn menu_show();
    fn menu_hide();
}

/// !is_warning = action
pub fn init() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let c = document
        .get_element_by_id("canvas")
        .expect("Where is my canvas?")
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    c.set_width(window.inner_width()?.as_f64().expect("number") as u32);
    c.set_height(window.inner_height()?.as_f64().expect("number") as u32);
    c.style().set_property("border", "solid")?;
    let mut attr = web_sys::ContextAttributes2d::new();
    let context = c
        .get_context_with_context_options("2d", attr.alpha(true))?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;
    context.set_image_smoothing_enabled(true);
    Ok(())
}
