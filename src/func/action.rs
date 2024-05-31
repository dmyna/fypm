//#region           Crates
use std::fs;
use std::str;
use std::process::Command;
use std::io::{Error, ErrorKind};
//#region           Modules
use crate::utils::get;
use crate::utils::structs::TaskWarriorExported;
use crate::utils::err::{FypmError, FypmErrorKind};
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
                        if let Some(next_task) = &next_json[0].seq_next {
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
