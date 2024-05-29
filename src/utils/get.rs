use std::{process::Command, str};

use super::err::{FypmError, FypmErrorKind};
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
                    kind: FypmErrorKind::NotEnoughArgs,
                });
            }
        }
    }

    Ok(parsed_json)
}