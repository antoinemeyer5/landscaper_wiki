mod components;
mod models;
mod pages;

use crate::pages::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
