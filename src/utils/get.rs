use std::{process::Command, str};

use super::{
    constants::DEFAULT_GET_JSON_OPTIONS, enums::TimewAction, err::{FypmError, FypmErrorKind}, structs
};
use crate::utils::structs::{GetJsonByFilterOptions, TaskWarriorExported, TimeWarriorExported};

pub fn get_json_by_filter(
    filter: &str,
    options: Option<GetJsonByFilterOptions>,
) -> Result<Vec<TaskWarriorExported>, FypmError> {
    let get_json = Command::new("task")
        .args([filter, "export"])
        .output()
        .unwrap()
        .stdout;
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
    let get_task = get_json_by_filter(&"+ACTIVE".to_string(), DEFAULT_GET_JSON_OPTIONS)?;
    let active_task = get_task.get(0);

    if active_task.is_none() {
        return Err(FypmError {
            message: "There is no active task!".to_string(),
            kind: FypmErrorKind::NoTasksFound,
        });
    } else {
        return Ok(active_task.unwrap().clone());
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
