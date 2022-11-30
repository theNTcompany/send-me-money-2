use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::Route;
use crate::components::layout::*;

#[function_component(Index)]
pub fn index() -> Html {
    html!(
        <AppLayout>
            <div class="px-8 py-8 md:px-16">
                <h1 class="text-2xl font-bold">{ "Welcome to My bank!" }</h1>
                <p class="pt-4">{ "Sign up for our saving account with 5% interest rate." }</p>
                <Link<Route> to={Route::SignIn}>
                    <button class="px-4 py-2 mt-8 rounded-xl bg-dark-blue hover:bg-blue transition-color duration-300">
                        { "Sign up!" }
                    </button>
                </Link<Route>>
            </div>
        </AppLayout>
    )
}
