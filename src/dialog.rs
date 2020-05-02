use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = dialog)]
    fn show();
    #[wasm_bindgen(js_namespace = dialog)]
    fn hide();
}
/* struct Dialog {
    dialog: web_sys::Element,
    overlay: web_sys::Element,
    name: web_sys::Element,
    container: web_sys::Element,
    options: web_sys::Element,
}

// TODO: to many unwrap, maybe handle errors with ?
impl Dialog {
    //const overlay = document.getElementById("over");
    //const dialog = document.getElementById("dialog");
    //dialog.name = document.querySelector("#dialog h1");
    //dialog.container = document.querySelector("#dialog .container");
    //dialog.options = document.querySelector("#dialog .options");
    /// This fn gives you dialog fns from everyware.
    pub fn get() -> Self {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let overlay = document.get_element_by_id("over").expect("Where is over?");
        let dialog = document
            .get_element_by_id("dialog")
            .expect("Where is over?");
        let name = document
            .query_selector("#dialog h1")
            .unwrap()
            .expect("Where is my name?");
        let container = document
            .query_selector("#dialog .container")
            .unwrap()
            .expect("Where is container?");
        let options = document
            .query_selector("#dialog .options")
            .unwrap()
            .expect("Where is option?");
        Dialog {
            dialog,
            overlay,
            name,
            container,
            options,
        }
    }
    // Here are real functions from js
    pub fn show(&self) {
        let window = web_sys::window().expect("no global `window` exists");
        self.container.set_inner_html("");
        self.options.set_inner_html("");
        // hoverballon style display none
        let overlay_style = self
            .overlay
            .dyn_ref::<HtmlElement>()
            .expect("#overlay should be an `HtmlElement`")
            .style();
        let dialog_style = self
            .dialog
            .dyn_ref::<HtmlElement>()
            .expect("#dialog should be an `HtmlElement`")
            .style();
        overlay_style.set_property("display", "block").unwrap();
        overlay_style.set_property("pointerEvents", "auto").unwrap();
        let closure: Closure<dyn FnMut(_)> = Closure::wrap(Box::new(
            move |overlay_style: web_sys::CssStyleDeclaration| {
                overlay_style.set_property("opacity", ".8").unwrap();
            },
        ));
        window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                closure.as_ref().unchecked_ref(),
                10,
            )
            .unwrap();
        closure.forget();
        dialog_style.set_property("display", "block").unwrap();
        let closure1: Closure<dyn FnMut(_)> = Closure::wrap(Box::new(
            move |dialog: web_sys::Element| {
                dialog
                    .dyn_ref::<HtmlElement>()
                    .expect("#dialog should be an `HtmlElement`")
                    .focus()
                    .unwrap();
                let dialog_style = dialog
                    .dyn_ref::<HtmlElement>()
                    .expect("#dialog should be an `HtmlElement`")
                    .style();
                dialog_style.set_property("opacity", "1").unwrap();
                dialog_style.set_property("transform", "scale(1)").unwrap();
                dialog_style.set_property("top", "16%").unwrap();
            }, //spam to keep fmt
        ));
        window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                closure1.as_ref().unchecked_ref(),
                10,
            )
            .unwrap();
        closure1.forget();
    }
    pub fn hide(&self) {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let overlay_style = self
            .overlay
            .dyn_ref::<HtmlElement>()
            .expect("#overlay should be an `HtmlElement`")
            .style();
        let dialog_style = self
            .dialog
            .dyn_ref::<HtmlElement>()
            .expect("#dialog should be an `HtmlElement`")
            .style();
        overlay_style.set_property("opacity", "0").unwrap();
        overlay_style.set_property("pointerEvents", "none").unwrap();
        let closure: Closure<dyn FnMut(_)> = Closure::wrap(Box::new(
            move |overlay_style: web_sys::CssStyleDeclaration| {
                if overlay_style.get_property_value("opacity").unwrap() == "0" {
                    overlay_style.set_property("display", "none").unwrap();
                }
            },
        ));
        window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                closure.as_ref().unchecked_ref(),
                500,
            )
            .unwrap();
        closure.forget();
        dialog_style.set_property("opacity", "0").unwrap();
        dialog_style.set_property("top", "100%").unwrap();
        let closure1: Closure<dyn FnMut(_)> = Closure::wrap(Box::new(
            move |dialog_style: web_sys::CssStyleDeclaration| {
                if dialog_style.get_property_value("opacity").unwrap() == "0" {
                    dialog_style.set_property("display", "none").unwrap();
                }
            },
        ));
        window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                closure1.as_ref().unchecked_ref(),
                200,
            )
            .unwrap();
        closure1.forget();
        document
            .get_element_by_id("canvas")
            .expect("Where is my canvas?")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap()
            .focus()
            .unwrap();
    }
    pub fn add_option(&self, text: String, onclick: &js_sys::Function) {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let button = document
            .create_element("button")
            .unwrap()
            .dyn_into::<web_sys::HtmlElement>()
            .unwrap();
        button.set_inner_html(&text);
        button.set_onmousedown(Some(onclick));
        let closure: Closure<dyn FnMut(_)> = Closure::wrap(Box::new(|_: String| {
            Dialog::get().hide();
        }));
        button.set_onmouseup(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
        self.options.append_child(&button).unwrap();
    }
    pub fn init(&self) {
        let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            if event.key() == "Enter" {
                Dialog::get()
                    .options
                    .children()
                    .item(Dialog::get().options.children().length() - 1)
                    .unwrap()
                    .dyn_ref::<HtmlElement>()
                    .expect("#overlay should be an `HtmlElement`")
                    .onmousedown()
                    .unwrap()
                    .call0(&wasm_bindgen::JsValue::NULL)
                    .unwrap();
            } else if event.key() == "Escape" {
                Dialog::get().hide();
            }
        }) as Box<dyn FnMut(_)>);
        self.dialog
            .clone()
            .dyn_into::<web_sys::HtmlElement>()
            .unwrap()
            .set_onkeydown(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }
} */