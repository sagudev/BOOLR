use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

pub static mut HIDE_TOOLBAR_MESSAGE: i32 = 0xAABBCCDD;

#[wasm_bindgen]
/// !is_warning = action
pub fn message(msg: String, is_warning: bool) -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let toast = document
        .get_element_by_id("toast")
        .expect("Where is my toaster?");
    let toast_style = toast
        .dyn_ref::<HtmlElement>()
        .expect("#loading should be an `HtmlElement`")
        .style();
    toast_style.set_property("display", "block")?;
    if is_warning {
        toast.set_inner_html(&format!(
            "<span class='material-icons' style='opacity: .5'>warning</span>{}",
            msg
        ));
    } else {
        toast.set_inner_html(&format!("{}<button onclick='undo()' style='font-family: Ubuntu'><span class='material-icons'>undo</span>Undo</button>", msg));
    }
    toast_style.set_property("marginLeft", &format!("{}px", -toast.client_width() / 2))?;
    toast_style.set_property("opacity", "1")?;
    let closure: Closure<dyn FnMut(_)> =
        Closure::wrap(Box::new(move |osc: web_sys::OscillatorNode| {
            toast_style.set_property("opacity", "0");
        }));
    unsafe {
        HIDE_TOOLBAR_MESSAGE = window.set_timeout_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            3000,
        )?;
    }
    closure.forget();
    Ok(())
}
