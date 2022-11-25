use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::Route;

#[function_component(NavBar)]
pub fn navbar() -> Html {
    html!(
      <div class="nav__wrapper">
        <Link<Route> to={Route::Home}>
          <h2>{ "My bank" }</h2>
        </Link<Route>>
        <nav>
          <Link<Route> to={Route::SignIn}>{ "My account" }</Link<Route>>
        </nav>
      </div>
    )
}
