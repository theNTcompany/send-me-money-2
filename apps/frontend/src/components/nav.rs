use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::Route;
use crate::components::icons::*;
use crate::utils::password::get_password;

#[function_component(NavBar)]
pub fn navbar() -> Html {
    let is_signed_in = get_password() == Some("password123".into());

    html!(
      <div class="flex items-center w-full px-8 py-4 bg-black text-white">
        <Link<Route> to={Route::Home} classes="hover:text-blue duration-300 transition-colors">
          <h2>{ "My bank" }</h2>
        </Link<Route>>
        <nav class="ml-auto">
          <div class="flex items-center gap-2 hover:text-blue duration-300 transition-colors">
            if !is_signed_in {
              <Link<Route> to={Route::SignIn} classes="flex items-center gap-2">
                <div class="w-4 h-4">
                  <LockIcon />
                </div>
                { "Sign in" }
              </Link<Route>>
            } else {
              <Link<Route> to={Route::Dashboard} classes="flex items-center gap-2">
                <div class="w-4 h-4">
                  <LockIcon />
                </div>
                { "Bob Smith" }
              </Link<Route>>
            }
          </div>
        </nav>
      </div>
    )
}
