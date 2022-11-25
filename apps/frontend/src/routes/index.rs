use yew::prelude::*;

use crate::components::layout::*;

#[function_component(Index)]
pub fn index() -> Html {
    html!(
        <AppLayout>
            <h1>{ "Welcome to my bank!" }</h1>
        </AppLayout>
    )
}
