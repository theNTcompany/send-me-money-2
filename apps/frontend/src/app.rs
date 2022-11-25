use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::index::*;
use crate::routes::sign_in::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/sign/in")]
    SignIn,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html!(<Index />),
        Route::SignIn => html!(<SignIn />),
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html!(
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    )
}
