use std::collections::HashMap;
use std::{process::Command, str, vec};

use chrono::NaiveDate;
use uuid::Uuid;

use crate::commands::TimewAction;

use fypm_lib::values::structs::{GetJsonByFilterOptions, TaskWarriorExported, TimeWarriorExported};
use fypm_lib::values::{
    err::{FypmError, FypmErrorKind},
    structs,
};

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

    serde_json::from_str::<Vec<structs::TimeWarriorExported>>(str_json)
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
