use yew::prelude::*;

#[function_component(SideBar)]
pub fn side_bar() -> Html {
    html! {
        <nav class="w-12 h-full bg-gray-950 flex items-center">
            <div class="flex justify-center h-12 w-full"> {"fy"} </div>
        </nav>
    }
}