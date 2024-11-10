
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            {"fypm is here."}
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
