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
use std::{collections::HashMap, error::Error, str::FromStr};
use yew::{prelude::*, virtual_dom::VNode};

use fypm_lib::values::structs::{TaskWarriorExported, TimeWarriorExported};
use fypm_lib::utils::date;

use crate::API_PORT;

/// Fetches the time logs from the local API and updates the provided state handle
/// with the rendered HTML list of time entries.
///
/// # Arguments
///
/// * `time_list` - A state handle to a virtual DOM node that will be updated with
///   the list of time entries fetched from the API. Each entry is represented as
///   an HTML list item displaying the task description or an error message if the
///   task UUID is not found in the task map.
pub async fn logs(
    time_list: UseStateHandle<VNode>,
    start: Option<&str>,
    end: Option<&str>,
) {
    let start_date = if let Some(start) = start {
        NaiveDate::from_str(date::match_aliases(&start.to_string()).as_str()).unwrap()
    } else {
        NaiveDate::from_str(date::match_aliases(&"today".to_string()).as_str()).unwrap()
    };

    let end_date = if let Some(end) = end {
        NaiveDate::from_str(date::match_aliases(&end.to_string()).as_str()).unwrap()
    } else {
        start_date + chrono::Duration::days(1)
    };

    let api_url = format!("http://localhost:{}/api", API_PORT.to_string());
    let url = format!("{}/time/log?start_date={}&end_date={}", api_url, start_date, end_date);

    let get_time_list = reqwest::get(url.as_str()).await;

    match get_time_list {
        Ok(response) => {
            let time_logs = serde_json::from_str::<(
                Vec<(String, TimeWarriorExported)>,
                HashMap<String, TaskWarriorExported>,
            )>(response.text().await.unwrap().as_str())
            .expect("Failed to parse time logs to json!");

            let mut entries: Vec<Html> = Vec::new();

            for time_log in time_logs.0 {
                let task = time_logs.1.get(&time_log.0);
                let description: String;

                match task {
                    Some(task) => {
                        description = task.description.clone();
                    }
                    None => {
                        description = format!("Task UUID {} not found", time_log.0);
                    }
                }

                entries.push(html! {
                    <li class="text-white">
                    {
                        description
                    }
                    </li>
                });
            }

            time_list.set(html! {
                <div id = "time-list">
                    <ul>
                    {
                        for entries.clone()
                    }
                    </ul>
                </div>
            });
        }
        Err(err) => {
            time_list.set(html! {
                <div id = "time-list" class="error">
                    {
                        format!(
                            "Err while getting time list:\n{}",
                            err.source().unwrap().to_string()
                        )
                    }
                </div>
            });
        }
    }
}
