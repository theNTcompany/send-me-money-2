use yew::prelude::*;

use crate::components::icons::*;

#[function_component(AppFooter)]
pub fn app_footer() -> Html {
    html!(
        <footer class="mt-auto bg-light-gray px-8 py-4">
            <p class="inline-flex items-center gap-2">
                { "Made with" }
                <span>
                    <HeartIcon />
                </span>
            </p>
            <p>
                { "MIT (c) 2022 " }
                <a href="https://vospel.cz" target="_blank" rel="noreferrer noopener" class="text-dark-blue hover:text-blue transition-colors duration-300">{ "Marek Vospěl" }</a>
            </p>
        </footer>
    )
}
