#![feature(is_some_and)]
#![feature(let_chains)]

mod app;
mod components;
mod routes;
mod utils;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
