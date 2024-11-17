use yew::prelude::*;

use crate::components::day_header::DayHeader;
use crate::components::tasks_list::TasksList;

#[function_component]
pub fn Summary() -> Html {
    html! {
        <div id="summary" class="flex flex-col gap-6 py-6">
            <DayHeader />
            <TasksList />
        </div>
    }
}
