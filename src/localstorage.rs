/* Is used only for storing clipboard, settings and tips */
use wasm_bindgen::prelude::*;

// TODO: move all checks for functions to one js file and retur error from there or use pollyfil.
#[wasm_bindgen]
extern "C" {
    pub fn localStorageAvailable() -> bool;
}

pub fn set_local_storage() {
    if !localStorageAvailable() {
        crate::dialog::local_storage_error();
        return;
    }
}

pub fn get_local_storage() {
    if !localStorageAvailable() {
        //dialog.localStorage
        return;
    }
}
