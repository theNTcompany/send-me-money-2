use wasm_bindgen::JsValue;
use web_sys::console::error_1;
use web_sys::window;

#[inline]
pub fn set_password(value: &str) {
    let local_storage = match window() {
        Some(window) => match window.local_storage() {
            Ok(local_storage) => local_storage,
            Err(_) => None,
        },
        None => None,
    };

    let result = match local_storage {
        Some(storage) => storage.set_item("password", value),
        None => Err(JsValue::from("Could not save password into local storage")),
    };

    if let Err(error) = result {
        error_1(&error);
    }
}

#[inline]
pub fn get_password() -> Option<String> {
    let local_storage = match window() {
        Some(window) => match window.local_storage() {
            Ok(local_storage) => local_storage,
            Err(_) => None,
        },
        None => None,
    };

    let result = match local_storage {
        Some(storage) => storage.get_item("password"),
        None => Err(JsValue::from("Could not save password into local storage")),
    };

    match result {
        Ok(value) => match value {
            Some(s) => s.into(),
            None => None,
        },
        Err(error) => {
            error_1(&error);
            Some("".into())
        }
    }
}
