use std::{process::Command, str};

use crate::values::{
    enums::TimewAction,
    err::{FypmError, FypmErrorKind},
    structs,
};
use crate::values::structs::{GetJsonByFilterOptions, TaskWarriorExported, TimeWarriorExported};

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
pub fn get_timew_json_by_filter(
    filter: &String,
) -> Result<Vec<TimeWarriorExported>, serde_json::Error> {
    let get_task_info = Command::new("timew")
        .args([String::from("export"), filter.to_string()])
        .output()
        .expect("Failed to get timew json!");

    let str_json = str::from_utf8(&get_task_info.stdout).unwrap();

    serde_json::from_str::<Vec<structs::TimeWarriorExported>>(str_json)
}
pub fn get_timew_time(id: &String, action: &TimewAction) -> String {
    let get_task_json = get_timew_json_by_filter(id).unwrap();
    let task_json = get_task_json.get(0).unwrap();

    if let TimewAction::Start = action {
        task_json.start.clone()
    } else {
        task_json.end.clone().unwrap_or("now".to_string())
    }
}
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
