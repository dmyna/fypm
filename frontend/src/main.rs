use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

pub const API_PORT: u16 = 8000;

mod request;

#[function_component(App)]
fn app() -> Html {
    let time_list = use_state(|| html! { <div id="time-list"> {"Loading list..."} </div> });

    {
        let time_list = time_list.clone();

        use_effect_with((), move |_| {
            spawn_local(async move {
                get_time_list(time_list).await;
            });
            || ()
        })
    }

    html! {
        <div class="bg-black text-white font-normal">
            {(*time_list).clone()}
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
