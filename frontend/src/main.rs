pub const API_PORT: u16 = 8000;

mod request;
mod components;
mod routes;

fn main() {
    yew::Renderer::<components::app::App>::new().render();
}
