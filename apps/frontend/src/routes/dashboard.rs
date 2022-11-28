use crate::app::Route;
use gloo_net::http::Request;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::layout::*;
use crate::utils::password::get_password;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(msg: &str);
}

#[function_component(Dashboard)]
pub fn index() -> Html {
    let navigator = use_navigator().unwrap();

    if let Some(password) = get_password() {
        if password != "password123" {
            navigator.push(&Route::SignIn);
        }
    } else {
        navigator.push(&Route::SignIn);
    }

    let balance = use_state(|| Option::<String>::None);
    let amount_input = use_node_ref();

    {
        let balance = balance.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let result = Request::get("/api/balance")
                        .header("Authorization", &get_password().unwrap_or("".into()))
                        .send()
                        .await;

                    if let Ok(result) = result && let Ok(result) = result.text().await {
                        balance.set(Some(result))
                    }
                });
                || ()
            },
            (),
        );
    }

    let onsubmit = {
        let amount_input = amount_input.clone();
        let balance = balance.clone();
        Callback::from(move |event: SubmitEvent| {
            let amount_input = amount_input.clone();
            let balance = balance.clone();
            event.prevent_default();

            let amount = match amount_input.cast::<HtmlInputElement>() {
                Some(input) => input.value(),
                None => "0".into(),
            };

            wasm_bindgen_futures::spawn_local(async move {
                log(&format!("{}", amount));
                let result = Request::post("/api/send")
                    .header("Authorization", &get_password().unwrap_or("".into()))
                    .body(format!("{{\"amount\": {}}}", amount))
                    .send()
                    .await;

                if let Ok(result) = result && let Ok(result) = result.text().await {
                    balance.set(Some(result))
                }
            });
        })
    };

    html!(
        <AppLayout>
            <form {onsubmit} class="generic__form">
                <p style="width: 100%;">
                    { "Balance: "}
                    {
                        if let Some(balance) = &*balance {
                            &balance
                        } else {
                            "Loading..."
                        }
                    }
                </p>
                <input placeholder="Amount" ref={amount_input} />
                <button type="submit">{ "Send" }</button>
            </form>
        </AppLayout>
    )
}
