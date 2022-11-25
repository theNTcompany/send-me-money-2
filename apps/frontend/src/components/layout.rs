use yew::prelude::*;

use crate::components::nav::*;

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    pub children: Children,
}

#[function_component(AppLayout)]
pub fn app_layout(props: &LayoutProps) -> Html {
    html!(
        <div>
            <NavBar />
            <main>
                { for props.children.iter() }
            </main>
        </div>
    )
}
