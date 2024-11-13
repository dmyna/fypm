use std::{collections::HashMap, error::Error};
use yew::{prelude::*, virtual_dom::VNode};

use fypm_lib::values::structs::{TaskWarriorExported, TimeWarriorExported};

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
pub async fn get_time_list(time_list: UseStateHandle<VNode>) {
    let api_url = format!("http://localhost:{}/api", API_PORT.to_string());
    let url = format!("{}/time/today/tomorrow", api_url);

    let get_time_list = reqwest::get(url.as_str()).await;

    match get_time_list {
        Ok(response) => {
            let time_logs = serde_json::from_str::<(
                Vec<(String, TimeWarriorExported)>,
                HashMap<String, TaskWarriorExported>,
            )>(response.text().await.unwrap().as_str())
            .unwrap();

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