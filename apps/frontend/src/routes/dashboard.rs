use crate::app::Route;
use gloo_net::http::Request;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::icons::*;
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

                if let Ok(result) = result && result.status() == 200 && let Ok(result) = result.text().await {
                    balance.set(Some(result))
                }
            });
        })
    };

    html!(
        <AppLayout>
            <form {onsubmit} class="px-8 py-8 md:px-16 flex flex-col w-full">
                <div class="flex flex-row items-center border-2 border-solid border-blue px-4 py-2">
                    <div>
                        <p class="font-semibold">{ "Savings account" }</p>
                        <p class="text-sm font-light">{ "1234567890/1337" }</p>
                    </div>
                    <p class="ml-auto">
                        { "$" }
                        {
                            if let Some(balance) = &*balance {
                                &balance
                            } else {
                                "Loading..."
                            }
                        }
                    </p>
                </div>
                <p class="font-bold mt-8">{ "Counterparty" }</p>
                <div class="flex flex-row items-center border-2 border-solid border-blue px-4 py-2">
                    <div>
                        <p class="font-semibold">{ "William Smith" }</p>
                        <p class="text-sm font-light">{ "1111111111/1337" }</p>
                    </div>
                    <div class="ml-auto cursor-not-allowed">
                        <ForwardIcon />
                    </div>
                </div>
                <div class="mt-32 flex flex-col">
                    <p>{ "Amount (USD)" }</p>
                    <input class="text-right bg-light-gray border border-solid border-b-blue px-4 pt-2 rounded-t" ref={amount_input} />
                    <div class="ml-auto mt-4">
                        <button class="px-8 py-2 bg-dark-blue hover:bg-blue transition-colors duration-300 rounded-xl" type="submit">{ "Send" }</button>
                    </div>
                </div>
            </form>
        </AppLayout>
    )
}
