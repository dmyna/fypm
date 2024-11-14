pub const API_PORT: u16 = 8000;

mod request;
mod components;

fn main() {
    yew::Renderer::<components::app::App>::new().render();
}
