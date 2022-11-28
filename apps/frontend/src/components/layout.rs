use yew::prelude::*;

use crate::components::footer::*;
use crate::components::nav::*;

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    pub children: Children,
}

#[function_component(AppLayout)]
pub fn app_layout(props: &LayoutProps) -> Html {
    html!(
        <div class="min-h-screen flex flex-col">
            <NavBar />
            <main>
                { for props.children.iter() }
            </main>
            <AppFooter />
        </div>
    )
}
