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

use chrono::NaiveDate;
use fypm_lib::utils::date;
use web_sys::HtmlInputElement;
use std::str::FromStr;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew::use_effect_with;

use crate::request;

#[derive(PartialEq, Properties)]
pub struct SelectDayProps {
    pub initial_date: String,
    pub on_date_changing: Callback<String>,
}

#[function_component]
pub fn SelectDay(props: &SelectDayProps) -> Html {
    let date_state = use_state(|| props.initial_date.clone());

    let onclick_minus = {
        let date_state = date_state.clone();
        let on_date_changing_emitter = props.on_date_changing.clone();

        move |_| {
            let date =
                NaiveDate::from_str(date::match_aliases(&date_state.to_string()).as_str()).unwrap();
            let new_date = date - chrono::Duration::days(1);

            date_state.set(new_date.to_string());

            on_date_changing_emitter.emit((*date_state).clone());
        }
    };
    let onclick_plus = {
        let date_state = date_state.clone();
        let on_date_changing_emitter = props.on_date_changing.clone();

        move |_| {
            let date =
                NaiveDate::from_str(date::match_aliases(&date_state.to_string()).as_str()).unwrap();
            let new_date = date + chrono::Duration::days(1);

            date_state.set(new_date.to_string());

            on_date_changing_emitter.emit((*date_state).clone());
        }
    };

    let onkeydown_input = {
        let date_state = date_state.clone();
        let on_date_changing_emitter = props.on_date_changing.clone();

        move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                date_state.set(e.target_dyn_into::<HtmlInputElement>().unwrap().value());

                on_date_changing_emitter.emit((*date_state).clone());
            }
        }
    };

    html! {
        <div class="flex flex-row justify-center gap-6 py-3 rounded-md" >
            <button onclick={onclick_minus}>{"<"}</button>
            <input
                class="bg-gray-700 text-slate-100"
                onkeydown={onkeydown_input}
                value={(*date_state).clone()}
            />
            <button onclick={onclick_plus}>{">"}</button>
        </div>
    }
}

#[function_component]
pub fn TimeList() -> Html {
    let time_list = use_state(|| {
        html! {
            <div> {"Loading list..."} </div>
        }
    });

    {
        let time_list = time_list.clone();

        use_effect_with((), move |_| {
            spawn_local(async move {
                request::time::logs(time_list, None, None).await;
            });
            || ()
        })
    }

    let time_list_bind = time_list.clone();

    let on_date_changing_callback = Callback::from(move |start: String| {
        let on_changing_time_list = time_list_bind.clone();

        spawn_local(async move {
            request::time::logs(
            on_changing_time_list,
                Some(start.as_str()),
                Some(
                    (NaiveDate::from_str(start.as_str()).unwrap() + chrono::Duration::days(1))
                        .to_string()
                        .as_str(),
                ),
            )
            .await;
        });
    });

    html! {
        <div>
            <SelectDay on_date_changing={on_date_changing_callback} initial_date="today"></SelectDay>
            <div class="bg-gray-800 rounded-3xl p-5">
                {(*time_list).clone()}
            </div>
        </div>
    }
}
