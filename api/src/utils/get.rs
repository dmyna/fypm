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
use std::collections::HashMap;
use std::process::Command;
use std::str;
use uuid::Uuid;

use fypm_lib::values::err::{FypmError, FypmErrorKind};
use fypm_lib::values::structs::{GetJsonByFilterOptions, TaskWarriorExported, TimeWarriorExported};

/// Retrieves a list of tasks from TaskWarrior based on the specified filter.
/// Optionally, additional overrides can be applied to the command arguments.
///
/// # Arguments
///
/// * `filter` - A string slice that holds the filter criteria for the tasks.
/// * `options` - An optional `GetJsonByFilterOptions` struct that may contain
///               additional overrides and a quantity constraint.
///
/// # Returns
///
/// * `Result<Vec<TaskWarriorExported>, FypmError>` - A result containing a vector
///   of `TaskWarriorExported` structs if successful, or a `FypmError` if an error occurs.
///
/// # Errors
///
/// * `FypmErrorKind::TooMuchTasks` - If the number of tasks retrieved exceeds the specified quantity.
/// * `FypmErrorKind::NoTasksFound` - If the number of tasks retrieved is less than the specified quantity.
///
/// # Panics
///
/// This function will panic if the command execution fails or if JSON parsing fails.
pub fn json_by_filter(
    filter: &str,
    options: Option<GetJsonByFilterOptions>,
) -> Result<Vec<TaskWarriorExported>, FypmError> {
    let mut args = Vec::new();

    if let Some(options) = &options {
        if let Some(overrides) = &options.aditional_overrides {
            args.extend(overrides.clone());
        }
    }
    args.extend(vec![filter.to_string(), "export".to_string()]);

    let get_json = Command::new("task").args(args).output().unwrap().stdout;

    let parsed_json =
        serde_json::from_str::<Vec<TaskWarriorExported>>(str::from_utf8(&get_json).unwrap())
            .unwrap();

    if let Some(options) = options {
        if let Some(quantity) = options.quantity {
            if parsed_json.len() > quantity {
                return Err(FypmError {
                    message: format!("Too much tasks! (expected: {})", quantity.to_string()),
                    kind: FypmErrorKind::TooMuchTasks,
                });
            } else if parsed_json.len() < quantity {
                return Err(FypmError {
                    message: format!("Not enough tasks! (expected: {})", quantity.to_string()),
                    kind: FypmErrorKind::NoTasksFound,
                });
            }
        }
    }

    Ok(parsed_json)
}
/// Retrieves timewarrior records from a given filter.
///
/// # Arguments
///
/// * `filter` - A vector of strings that will be passed as arguments to `timew export`.
///
/// # Returns
///
/// * `Result<Vec<TimeWarriorExported>, serde_json::Error>` - A result containing a vector of
///   `TimeWarriorExported` structs if successful, or a `serde_json::Error` if the JSON parsing
///   fails.
pub fn get_timew_json_by_filter(
    filter: &Vec<String>,
) -> Result<Vec<TimeWarriorExported>, serde_json::Error> {
    let mut args = vec!["export"];
    args.extend(filter.iter().map(|f| f.as_str()));

    let get_task_info = Command::new("timew")
        .args(args)
        .output()
        .expect("Failed to get timew json!");

    let str_json = str::from_utf8(&get_task_info.stdout).unwrap();

    serde_json::from_str::<Vec<TimeWarriorExported>>(str_json)
}
/// Get all timew entries between a start and end date, and their respective tasks as a HashMap.
///
/// # Errors
/// If there is an error parsing the UUID from the timewarrior tag, it will propagate the error.
pub fn timew_entries(
    start: NaiveDate,
    end: NaiveDate,
) -> Result<
    (
        Vec<(String, TimeWarriorExported)>,
        HashMap<String, TaskWarriorExported>,
    ),
    FypmError,
> {
    let timew_json =
        get_timew_json_by_filter(&vec![start.to_string(), "-".to_string(), end.to_string()])
            .unwrap();

    let mut timew_entries: Vec<(String, TimeWarriorExported)> = Vec::new();

    for timew_entry in timew_json {
        for tag in timew_entry.tags.as_ref().unwrap_or(&vec![]) {
            match Uuid::parse_str(&tag) {
                Ok(uuid) => {
                    timew_entries.push((uuid.to_string(), timew_entry.clone()));
                }
                Err(_) => {}
            }
        }
    }

    let tasks_json = json_by_filter(
        timew_entries
            .iter()
            .map(|timew_entry| timew_entry.0.clone())
            .collect::<Vec<_>>()
            .join(" ")
            .as_str(),
        None,
    )?;

    let tasks_map = tasks_json
        .iter()
        .map(|task| (task.uuid.clone(), task.clone()))
        .collect::<HashMap<String, TaskWarriorExported>>();

    Ok((timew_entries, tasks_map))
}
