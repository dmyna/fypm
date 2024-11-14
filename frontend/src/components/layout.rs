use yew::prelude::*;

use super::sidebar::SideBar;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub children: Html,
}

#[function_component(Layout)]
pub fn layout(props: &Props) -> Html {
    html! {
        <div class="h-screen bg-gray-900 text-white font-normal flex flex-row">
            <SideBar></SideBar>
            <main id="main-content" class="h-full px-12 overflow-auto w-full">
                {props.children.clone()}
            </main>
        </div>
    }
}