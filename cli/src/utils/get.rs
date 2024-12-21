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

use std::{process::Command, str, vec};

use crate::commands::TimewAction;

use fypm_lib::values::err::{FypmError, FypmErrorKind};
use fypm_lib::values::structs::{GetJsonByFilterOptions, TaskWarriorExported};

/// Retrieves the active task from TaskWarrior.
///
/// # Returns
///
/// * `Result<TaskWarriorExported, FypmError>` - A result containing the active task if successful,
///   or a `FypmError` if an error occurs or there is no active task.
///
/// # Errors
///
/// * `FypmErrorKind::NoTasksFound` - If there is no active task.
pub fn get_current_task_json() -> Result<TaskWarriorExported, FypmError> {
    let get_task = json_by_filter("+ACTIVE", None)?;
    let active_task = get_task.get(0);

    if let Some(active) = active_task {
        Ok(active.clone())
    } else {
        Err(FypmError {
            message: "There is no active task!".to_string(),
            kind: FypmErrorKind::NoTasksFound,
        })
    }
}
/// Retrieves the start or end time of a timewarrior task.
///
/// # Arguments
///
/// * `id` - The id of the task.
/// * `action` - The type of time to retrieve. Use `TimewAction::Start` to get the start time or `TimewAction::End` to get the end time.
///
/// # Returns
///
/// * `String` - The time of the task in ISO format. If the task doesn't have an end time, it will return `"now"`.
pub fn get_timew_time(id: &String, action: &TimewAction) -> String {
    let get_task_json = get_timew_json_by_filter(&vec![id.clone()]).unwrap();
    let task_json = get_task_json.get(0).unwrap();

    if let TimewAction::Start = action {
        task_json.start.clone()
    } else {
        task_json.end.clone().unwrap_or("now".to_string())
    }
}
/// Retrieves the UUIDs of tasks from TaskWarrior that match a given filter.
///
/// # Arguments
///
/// * `filter` - A string slice that holds the filter criteria for the tasks.
/// * `options` - An optional `GetJsonByFilterOptions` struct that may contain
///               additional overrides and a quantity constraint.
///
/// # Returns
///
/// * `Result<Vec<String>, FypmError>` - A result containing a vector of UUIDs if successful,
///   or a `FypmError` if an error occurs.
///
/// # Errors
///
/// * `FypmErrorKind::TooMuchTasks` - If the number of tasks retrieved exceeds the specified quantity.
/// * `FypmErrorKind::NoTasksFound` - If the number of tasks retrieved is less than the specified quantity.
///
/// # Panics
///
/// This function will panic if the command execution fails.
pub fn get_uuids_by_filter(
    filter: &str,
    options: Option<GetJsonByFilterOptions>,
) -> Result<Vec<String>, FypmError> {
    let uuids = String::from_utf8_lossy(
        &Command::new("task")
            .args([filter, "_uuids"])
            .output()
            .unwrap()
            .stdout,
    )
    .lines()
    .map(|line| line.to_string())
    .collect::<Vec<String>>();

    if let Some(options) = options {
        if let Some(quantity) = options.quantity {
            if uuids.len() > quantity {
                return Err(FypmError {
                    message: format!("Too much tasks! (expected: {})", quantity.to_string()),
                    kind: FypmErrorKind::TooMuchTasks,
                });
            } else if uuids.len() < quantity {
                return Err(FypmError {
                    message: format!("Not enough tasks! (expected: {})", quantity.to_string()),
                    kind: FypmErrorKind::NoTasksFound,
                });
            }
        }
    }

    Ok(uuids)
}
/// Retrieves the number of tasks that matches the given filter.
///
/// # Arguments
///
/// * `filter` - A string slice that holds the filter criteria for the tasks.
///
/// # Returns
///
/// * `Result<u32, FypmError>` - A result containing the number of tasks if successful,
///   or a `FypmError` if an error occurs.
///
/// # Panics
///
/// This function will panic if the command execution fails or if the output is not a valid
/// number.
pub fn get_count_by_filter(filter: &String) -> Result<u32, FypmError> {
    let mut tasks_count: u32 = 0;

    let tasks_length = String::from_utf8(
        Command::new("task")
            .args([filter, &"count".to_string()])
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap();

    if tasks_length != "" {
        tasks_count = tasks_length.trim().parse::<u32>().unwrap();
    }

    Ok(tasks_count)
}
/// Retrieves the filter for a given TaskWarrior modifier.
///
/// # Arguments
///
/// * `modifier` - A string slice that holds the modifier name.
///
/// # Returns
///
/// * `Result<String, FypmError>` - A result containing the filter associated with the modifier if successful,
///   or a `FypmError` if an error occurs.
///
/// # Errors
///
/// * `FypmErrorKind::TaskWarriorError` - If the command execution fails.
pub fn filter_by_modifier(modifier: &String) -> Result<String, FypmError> {
    let cfg_key = format!("report.{modifier}.filter");

    let get_configs = String::from_utf8(
        Command::new("task")
            .args(["show", cfg_key.as_str()])
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap();

    let lines_with_config = get_configs
        .split("\n")
        .filter(|line| line.contains(cfg_key.as_str()))
        .collect::<Vec<&str>>()
        .join("\n");

    let config = lines_with_config
        .replace(cfg_key.as_str(), "")
        .trim()
        .to_string();

    Ok(config)
}

/// Retrieves the MOTHER task from TaskWarrior that matches a given Sequence ID.
///
/// # Arguments
///
/// * `seq_id` - The Sequence ID to search for.
///
/// # Returns
///
/// * `Result<TaskWarriorExported, FypmError>` - A result containing the MOTHER task if successful,
///   or a `FypmError` if an error occurs.
///
/// # Errors
///
/// * `FypmErrorKind::TooMuchTasks` - If the number of tasks retrieved exceeds the specified quantity.
/// * `FypmErrorKind::NoTasksFound` - If the number of tasks retrieved is less than the specified quantity.
/// * `FypmErrorKind::NotEnoughTasks` - If the number of tasks retrieved is not enough to get a MOTHER task.
///
pub fn mother_json_by_sequence_id(seq_id: &String) -> Result<TaskWarriorExported, FypmError> {
    let tasks = json_by_filter(format!("(status:pending and {})", seq_id).as_str(), None)?;

    if tasks.len() > 1 {
        let mut mother = (false, 0);
        for (index, task) in tasks.iter().enumerate() {
            if task.tags.as_ref().unwrap().contains(&"MOTHER".to_string()) {
                mother = (true, index);
            }
        }

        if mother.0 {
            let mother_task = tasks.get(mother.1).unwrap();

            if mother_task
                .tags
                .as_ref()
                .unwrap()
                .contains(&"Sequence".to_string())
            {
                Ok(mother_task.clone())
            } else {
                Err(FypmError {
                    message: format!(
                        "{}\n{}",
                        "You are trying to get a MOTHER that is not a Sequence. ",
                        "If you want to get a task by a tag, this task must be a Sequence. ",
                    ),
                    kind: FypmErrorKind::TooMuchTasks,
                })
            }
        } else {
            Err(FypmError {
                message: format!(
                    "{}\n{}",
                    "Not enough tasks! (expected: 1)",
                    "If you want to get a task by a tag, this task must be a Sequence. ",
                ),
                kind: FypmErrorKind::NotEnoughTasks,
            })
        }
    } else if tasks.len() <= 1 {
        Err(FypmError {
            message: "No tasks found! (expected: > 1)".to_string(),
            kind: FypmErrorKind::NoTasksFound,
        })
    } else {
        Err(FypmError {
            message: format!(
                "{}\n{}{}",
                "Not enough tasks! (expected: > 1)",
                "If you want to get a task by a tag, this task must be a Sequence. ",
                "If this sequence only have 1 task (mother), there's nothing to start.",
            ),
            kind: FypmErrorKind::NotEnoughTasks,
        })
    }
}
