#![allow(dead_code)]
extern crate js_sys;
extern crate wasm_bindgen;
extern crate web_sys;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use web_sys::HtmlElement;

mod audio;
mod dialog;
mod canvas;
mod components;
mod localstorage;
mod saved_custom_components;
mod toolbar;

// from startup.js
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    canvas::init()?;
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
