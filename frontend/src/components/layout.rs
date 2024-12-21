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