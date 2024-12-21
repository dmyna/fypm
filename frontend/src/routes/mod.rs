////////////////////////////////////////////////////////////////////////////////
// fypm - The Dark Souls of productivity.
// Copyright (C) 2023-2024 Rikagaku <contact.rikagaku@gmail.com>
// Copyright (C) 2023-2024 Myna <contact@devmyna.xyz>
//
// fypm is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// fypm is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with fypm. If not, see <https://www.gnu.org/licenses/>.
//
////////////////////////////////////////////////////////////////////////////////

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
