use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::Route;
use crate::components::layout::*;
use crate::utils::password::{get_password, set_password};

#[function_component(SignIn)]
pub fn sign_in() -> Html {
    let password_input = use_node_ref();
    let incorrect_password = use_state(|| false);
    let navigator = use_navigator().unwrap();

    if let Some(password) = get_password() && password == "password123" {
        navigator.push(&Route::Dashboard);
    }

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

            set_password(&password);

            navigator.push(&Route::Dashboard);
        })
    };

    html!(
        <AppLayout>
            <form {onsubmit} class="generic__form">
                <h1>{ "Sign in!" }</h1>
                <div>
                    <input type="password" placeholder="Password" ref={password_input} />
                    if *incorrect_password {
                        <p class="sign-in__error">{ "Incorrect password!" }</p>
                    }
                </div>
                <button type="submit">{ "Sign in" }</button>
            </form>
        </AppLayout>
    )
}
