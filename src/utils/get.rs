use std::{process::Command, str};

use super::{
    constants::DEFAULT_GET_JSON_OPTIONS,
    err::{FypmError, FypmErrorKind},
};
use crate::utils::structs::{GetJsonByFilterOptions, TaskWarriorExported};

pub fn get_json_by_filter(
    filter: &String,
    options: Option<GetJsonByFilterOptions>,
) -> Result<Vec<TaskWarriorExported>, FypmError> {
    let get_json = Command::new("task")
        .args([filter.as_str(), "export"])
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
                    kind: FypmErrorKind::TooMuchArgs,
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
    let active_task =  get_task.get(0);

    if active_task.is_none() {
        return Err(FypmError {
            message: "There is no active task!".to_string(),
            kind: FypmErrorKind::NoTasksFound,
        });
    } else {
        return Ok(active_task.unwrap().clone());
    }
}
