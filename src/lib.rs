#![allow(dead_code)]
#[macro_use]
extern crate serde_derive;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use web_sys::HtmlElement;

mod utils;
use crate::utils::set_panic_hook;

mod audio;
mod canvas;
mod components;
mod dialog;
mod localstorage;
mod saved_custom_components;
mod toolbar;

// from startup.js
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    set_panic_hook();
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    //canvas::init()?;
    /*
    getLocalStorage(localStorage.pwsData);
    getCustomComponents();

    readSaveFiles();
    updateDebugInfo();
    setInterval(updateDebugInfo, 500);
    draw();
    */
    document
        .query_selector(".main-menu .loading")?
        .expect("Where is .main-menu .loading")
        .dyn_ref::<HtmlElement>()
        .expect("#loading should be an `HtmlElement`")
        .style()
        .set_property("display", "none")?;
    let buttons = document.query_selector_all(".main-menu > button")?;
    for i in 0..buttons.length() {
        let el = buttons
            .item(i)
            .expect("no item in buttons")
            .dyn_ref::<HtmlElement>()
            .expect("#loading should be an `HtmlElement`")
            .style();
        el.set_property("top", "0")?;
        el.set_property("opacity", "1")?;
        el.set_property("transform", "translateX(0px)")?;
        buttons
            .item(i)
            .expect("no item in buttons")
            .dyn_ref::<Element>()
            .expect("#loading should be an `Element`")
            .query_selector(".material-icons")?
            .expect("not these")
            .dyn_ref::<HtmlElement>()
            .expect("#loading should be an `HtmlElement`")
            .style()
            .set_property("transform", "translateX(0px)")?;
    }
    Ok(())
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

mod console {
    use wasm_bindgen::prelude::wasm_bindgen;
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = console)]
        fn log(msg: String);
        #[wasm_bindgen(js_namespace = console)]
        fn warn(msg: String);
    }
    macro_rules! console_log {
        // Note that this is using the `log` function imported above during
        // `bare_bones`
        ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
    }
    macro_rules! console_warn {
        // Note that this is using the `log` function imported above during
        // `bare_bones`
        ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
    }
}
