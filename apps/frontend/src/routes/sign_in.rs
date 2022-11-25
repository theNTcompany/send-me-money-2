use wasm_bindgen::prelude::*;
use web_sys::console::log_1;
use web_sys::{window, HtmlInputElement};
use yew::prelude::*;

use crate::components::layout::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[function_component(SignIn)]
pub fn sign_in() -> Html {
    let password_input = use_node_ref();
    let incorrect_password = use_state(|| false);

    let onsubmit = {
        let password_input = password_input.clone();
        let incorrect_password = incorrect_password.clone();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            incorrect_password.set(false);
            let password = match password_input.cast::<HtmlInputElement>() {
                Some(input) => input.value(),
                None => "".into(),
            };

            if password != "password123" {
                return incorrect_password.set(true);
            }

            let local_storage = match window() {
                Some(window) => match window.local_storage() {
                    Ok(local_storage) => local_storage,
                    Err(_) => None,
                },
                None => None,
            };

            let result = match local_storage {
                Some(storage) => storage.set_item("password", &password),
                None => Err(JsValue::from("Clould not save password into local storage")),
            };

            if let Err(error) = result {
                log_1(&error);
            }
        })
    };

    html!(
        <AppLayout>
            <form {onsubmit} class="sign-in__form">
                <h1>{ "Sign in!" }</h1>
                <input type="password" placeholder="Password" ref={password_input} />
                { *incorrect_password }
                <button type="submit">{ "Sign in" }</button>
            </form>
        </AppLayout>
    )
}
