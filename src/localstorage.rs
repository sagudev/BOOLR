/* Is used only for storing clipboard, settings and tips */
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn localStorageAvailable() -> bool;
}

pub fn set_local_storage() {}

pub fn get_local_storage() {}
