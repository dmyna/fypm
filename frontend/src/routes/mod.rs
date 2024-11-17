use yew::prelude::*;
use yew_router::{BrowserRouter, Routable, Switch};

pub mod logs;
pub mod summary;

use logs::Logs;
use summary::Summary;

#[derive(Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/summary")]
    Summary,
    #[at("/logs")]
    Logs,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component]
pub fn Router() -> Html {
    fn switch_route(routes: Route) -> Html {
        match routes {
            Route::Home => html! { <h1> {"Home"} </h1> },
            Route::Summary => html! { <Summary/> },
            Route::Logs => html! { <Logs/> },
            Route::NotFound => html! { <h1> {"404"} </h1> },
        }
    }

    html! {
        <BrowserRouter>
            <Switch<Route> render={switch_route} />
        </BrowserRouter>
    }
}
