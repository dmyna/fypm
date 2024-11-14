use yew::prelude::*;

use crate::routes::Router;
use super::layout::Layout;

#[function_component]
pub fn App() -> Html {
    html! {
        <div>
            <Layout>
                <Router />
            </Layout>
        </div>
    }
}