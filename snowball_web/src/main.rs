mod app;
mod app_state;
mod components;
mod screens;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
