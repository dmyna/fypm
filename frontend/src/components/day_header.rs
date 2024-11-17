use yew::prelude::*;

#[function_component]
pub fn DayHeader() -> Html {
    let current_day = chrono::Local::now().date_naive();

    html! {
        <div class="">
            <div id="date-box"> {current_day.to_string()} </div>
        </div>
    }
}