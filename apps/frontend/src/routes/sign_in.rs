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
            <form {onsubmit} class="px-8 py-8 md:px-16 flex flex-col w-full">
                // TODO: add an art :)
                <div class="w-full md:ml-auto md:pr-64 md:w-auto">
                    <h1 class="text-2xl font-bold">{ "Sign in!" }</h1>
                    <div class="pt-4">
                        <input type="password" placeholder="Password" ref={password_input} class="bg-gray py-2 px-4 placeholder:text-white rounded-xl w-full sm:w-auto" />
                        if *incorrect_password {
                            <p class="text-warning">{ "Incorrect password!" }</p>
                        }
                    </div>
                    <button type="submit" class="bg-dark-blue hover:bg-blue mt-2 px-4 py-2 rounded-xl transition-colors duration-300 w-full sm:w-auto">{ "Sign in" }</button>
                </div>
            </form>
        </AppLayout>
    )
}
