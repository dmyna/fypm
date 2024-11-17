use yew::prelude::*;

use crate::components::time_list::TimeList;

#[function_component]
pub fn Logs() -> Html {
    html! {
        <div id="time-list">
            <TimeList />
        </div>
    }
}
