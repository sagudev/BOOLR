use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

pub static mut HIDE_TOOLBAR_MESSAGE: Option<i32> = None;

#[wasm_bindgen]
/// !is_warning = action
pub fn toolbar_message(msg: String, is_warning: bool) -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    unsafe {
        if let Some(htm) = HIDE_TOOLBAR_MESSAGE {
            window.clear_timeout_with_handle(htm);
        }
    }
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
    let closure: Closure<dyn FnMut(_)> = Closure::wrap(Box::new(
        move |toast_style: web_sys::CssStyleDeclaration| {
            toast_style.set_property("opacity", "0").unwrap();
        },
    ));
    unsafe {
        HIDE_TOOLBAR_MESSAGE = Some(
            window.set_timeout_with_callback_and_timeout_and_arguments_0(
                closure.as_ref().unchecked_ref(),
                3000,
            )?,
        );
    }
    closure.forget();
    Ok(())
}

/* #[wasm_bindgen]
pub fn list_show() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let list_style = document
        .get_element_by_id("list")
        .expect("Where is list?")
        .dyn_ref::<HtmlElement>()
        .expect("#loading should be an `HtmlElement`")
        .style();
    list_style.set_property("display", "block")?;
    let closure: Closure<dyn FnMut(_)> =
        Closure::wrap(Box::new(move |list_style: web_sys::CssStyleDeclaration| {
            list_style.set_property("opacity", "1").unwrap();
            list_style.set_property("transform", "scale(1)").unwrap();
        }));
    window.set_timeout_with_callback_and_timeout_and_arguments_0(
        closure.as_ref().unchecked_ref(),
        1,
    )?;
    closure.forget();
    Ok(())
}

#[wasm_bindgen]
pub fn list_hide() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let list_style = document
        .get_element_by_id("list")
        .expect("Where is list?")
        .dyn_ref::<HtmlElement>()
        .expect("#loading should be an `HtmlElement`")
        .style();
    list_style.set_property("opacity", "0")?;
    list_style.set_property("transform", "scale(.5) translateX(-63px) translateY(150px)")?;
    document
        .get_element_by_id("list")
        .expect("Where is list?")
        .dyn_ref::<HtmlElement>()
        .expect("#loading should be an `HtmlElement`")
        .focus()?;
    let closure: Closure<dyn FnMut(_)> =
        Closure::wrap(Box::new(move |list_style: web_sys::CssStyleDeclaration| {
            list_style.set_property("display", "none").unwrap();
        }));
    window.set_timeout_with_callback_and_timeout_and_arguments_0(
        closure.as_ref().unchecked_ref(),
        200,
    )?;
    closure.forget();
    Ok(())
} */
