//#region           Crates
use std::fs;
use std::str;
use std::process::Command;
use std::io::{Error, ErrorKind};
//#region           Modules
use crate::utils::get;
use crate::utils::err::{FypmError, FypmErrorKind};
use crate::utils::structs::{TaskWarriorExported};
use crate::utils::constants::{DEFAULT_GET_JSON_OPTIONS, LAST_TASK_PATH};

//#endregion
//#region           Implementation
pub fn annotate(command: &str, id: &String, annotation: &String) {
    let execute = Command::new("timew")
        .args([command, id, annotation])
        .output()
        .unwrap();

    println!("{}", str::from_utf8(&execute.stdout).unwrap());
}

pub fn receive_last_task() -> Result<String, Error> {
    let get_last_task = fs::read(LAST_TASK_PATH)?;

    let last_task = String::from_utf8(get_last_task);

    if last_task.is_ok() {
        Ok(last_task.unwrap())
    } else {
        Err(Error::new(
            std::io::ErrorKind::InvalidData,
            last_task.err().unwrap(),
        ))
    }
}
pub fn match_special_aliases(filter: &String) -> String {
    match filter.as_str() {
        // Last Task
        "last" => receive_last_task().unwrap(),
        // Time without specific use
        "t" => "5c847c7e-c7eb-44f6-ad7e-29cc989c8854".to_string(),
        // Lost time
        "l" => "1469ac5d-78ab-463d-bf77-f56a9f042f48".to_string(),
        // Rest and breaks
        "d" => "309d9b37-cd99-4b2c-b3c7-a9c60cb1754f".to_string(),
        // Hygiene and Selfcare
        "h" => "a371cb4e-6fad-452f-a22c-abc932f0a83f".to_string(),
        // Singing
        "s" => "2d5d97b5-fe43-415f-8501-045aca46cdbb".to_string(),
        // Active Thought || DNM
        "p" => "dd67efbb-f010-42c7-b84c-5d0da1936e57".to_string(),
        // Calisthenics and Stretching
        "e" => "7806d5f7-db60-4841-ba83-97c2106499d3".to_string(),
        // Chess Practice
        "x" => "100372a8-5ca2-493a-b6f3-4b74195c8848".to_string(),
        // House Maintening
        "hm" => "ef5dbc2c-326e-4443-b0dc-b2595de6e012".to_string(),
        // Workflow Maintening
        "wm" => "b719a399-0b21-4fed-9118-017096466073".to_string(),
        // Tasks Maintening
        "tm" => "8980c7be-1fda-4888-b45a-1a2e52345947".to_string(),
        _ => filter.to_string(),
        // Need to implement a filter to prevent cases like "r", "ab", etc.
        // Now, if I write "r", it will pass and break
    }
}
pub fn verify_if_wt_is_allday(filter_json: &TaskWarriorExported) -> Result<(), Error> {
    if filter_json.wt == "AllDay" {
        Err(Error::new(
            ErrorKind::InvalidInput,
            "You are trying to start a task that is AllDay!".to_string(),
        ))
    } else {
        Ok(())
    }
}
pub fn verify_if_is_divisory(filter_json: &TaskWarriorExported) -> Result<(), Error> {
    if let Some(tags) = &filter_json.tags {
        if tags.contains(&"Divisory".to_string()) {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "The specified task is a Divisory!".to_string(),
            ));
        }
    }

    Ok(())
}
pub fn match_inforelat_and_sequence(filter_json: &TaskWarriorExported) -> Result<String, FypmError> {
    let state = &filter_json.state;
    let r#type = &filter_json.r#type;

    let is_sequence: bool;
    if let Some(verify_tags) = &filter_json.tags {
        if verify_tags.contains(&"Sequence".to_string()) {
            is_sequence = true;
        } else {
            is_sequence = false;
        }
    } else {
        is_sequence = false;
    }

    if state == "Info" {
        let inforelat = &filter_json.inforelat;

        if let Some(inforelat) = inforelat {
            let new_filter_json = get::get_json_by_filter(&inforelat, DEFAULT_GET_JSON_OPTIONS).unwrap();

            return match_inforelat_and_sequence(&new_filter_json[0]);
        } else {
            if is_sequence {
                if let Some(next_task) = &filter_json.seq_current {
                    let mut next_json = get::get_json_by_filter(&next_task, DEFAULT_GET_JSON_OPTIONS)?;
                    let mut status = next_json[0].status.as_str();

                    // Loop until find a pending task or there is no next task

                    while status == "completed" {
                        if let Some(next_task) = &filter_json.seq_next {
                            next_json = get::get_json_by_filter(&next_task, DEFAULT_GET_JSON_OPTIONS)?;
                            status = next_json[0].status.as_str();
                        } else {
                            return Err(FypmError {
                                    kind: FypmErrorKind::NoTasksFound,
                                    message: "This task doesn't have a current task that is pending or even a next task. You should done it!".to_string()
                            });
                        }
                    }

                    return match_inforelat_and_sequence(&next_json[0]);
                } else {
                    Err(FypmError {
                        kind: FypmErrorKind::NoTasksFound,
                        message: "There is no next task.".to_string(),
                    })
                }
            } else {
                Err(FypmError {
                    kind: FypmErrorKind::WrongInitialization,
                    message: "You are trying to start a Info task without a INFORELAT!".to_string(),
                })
            }
        }
    } else {
        if is_sequence {
            if r#type == "SubTask" {
                Ok(filter_json.uuid.clone())
            } else {
                Err(FypmError {
                    kind: FypmErrorKind::ProblemWithStoredTask,
                    message: "Your Sequence task is not Informative and is not a SubTask. It shouldn't be happening!".to_string()
            })
            }
        } else {
            Ok(filter_json.uuid.clone())
        }
    }
}
//#endregion
