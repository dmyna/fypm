//#region           Crates
use std::fs;
use std::io::{Error, ErrorKind, Write};
use std::process::Command;
use std::process::Stdio;
use std::str;

use itertools::Itertools;

//#region           Modules
use crate::utils::get;
use fypm_lib::values::constants::{DEFAULT_GET_JSON_OPTIONS, LAST_TASK_PATH};
use fypm_lib::values::err::{FypmError, FypmErrorKind};
use fypm_lib::values::structs::{TaskWarriorExported, TaskWarriorStatus};

use super::command;
//#endregion
//#region           Implementation

/// Annotates a task with given text.
///
/// # Arguments
/// * `command`: The command to be executed, e.g. "task" or "timew".
/// * `filter`: The filter to select the task to annotate.
/// * `annotation`: The annotation to be added to the task.
/// * `skip_confirmation`: If true, the annotation will be added without confirmation.
///
/// # Errors
/// Returns `FypmError` if an error occurs while executing the command.
pub fn annotate(
    command: &str,
    filter: &String,
    annotation: &String,
    skip_confirmation: bool,
) -> Result<(), FypmError> {
    let mut args = Vec::new();
    {
        args.extend(["rc.verbose=0", "rc.recurrence.confirmation=off"]);

        if skip_confirmation {
            args.extend(["rc.confirmation=off"]);
        }
        args.extend([filter, "annotate", annotation]);
    }

    let mut binding = Command::new(command);
    let mut execute = binding
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    if skip_confirmation {
        execute = execute.stdin(Stdio::piped());

        let get_count = Command::new("task")
            .args(["rc.verbose=0", filter, "count"])
            .output()
            .unwrap();
        let tasks_count = String::from_utf8(get_count.stdout)
            .unwrap()
            .trim()
            .parse::<u32>()
            .unwrap();

        if tasks_count > 2 {
            let mut child = execute.spawn().unwrap();

            let mut stdin = child.stdin.take().unwrap();
            stdin.write_all("all\n".as_bytes()).unwrap();

            child.wait().unwrap();

            return Ok(());
        }
    } else {
        execute = execute.stdin(Stdio::inherit());
    }

    execute.output().unwrap();

    Ok(())
}
/// Unarchives a list of tasks by modifying their status to pending.
///
/// # Arguments
/// * `tasks` - A vector of `TaskWarriorExported` representing the tasks to be unarchived.
///
/// # Returns
/// * `Result<(), FypmError>` - Returns an `Ok(())` if successful, or a `FypmError` if an error occurs.
///
/// # Errors
/// Returns `FypmError` if an error occurs while executing the unarchive command.
///
/// The function will execute the command to modify the tasks' status to pending and remove the "Archived" tag.
/// If there are more than two tasks, it will use `stdin_all` for batch processing.
pub fn unarchive(tasks: Vec<TaskWarriorExported>) -> Result<(), FypmError> {
    let mut modify_binding = Command::new("task");
    let modify_command = modify_binding
        .args(vec![
            "rc.verbose=0",
            "rc.confirmation=0",
            "rc.recurrence.confirmation=0",
            tasks
                .iter()
                .map(|task| task.uuid.clone())
                .join(" ")
                .as_str(),
            "modify",
            "status:pending",
            "-Archived",
        ])
        .stderr(Stdio::inherit());

    if tasks.len() > 2 {
        command::stdin_all(modify_command).unwrap();
    } else {
        modify_command.output().unwrap();
    }

    Ok(())
}
/// Reads the last task from the file at `LAST_TASK_PATH` and returns it as a `String`.
///
/// # Returns
/// * `Result<String, Error>` - Returns `Ok(String)` if successful, or `Err(Error)` if the file could not be read or the contents could not be parsed as a `String`.
///
/// # Errors
/// Returns `Error` if an error occurs while reading the file or parsing the contents as a `String`.
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
/// Verify if the task is allday.
///
/// # Errors
/// If true, it will return an error warning that you are trying to start a task that is AllDay.
pub fn verify_if_wt_is_allday(json: &TaskWarriorExported) -> Result<(), Error> {
    if json.wt == "AllDay" {
        Err(Error::new(
            ErrorKind::InvalidInput,
            "You are trying to start a task that is AllDay!".to_string(),
        ))
    } else {
        Ok(())
    }
}
/// Verify if the task is a Divisory.
///
/// # Errors
/// If true, it will return an error warning that you are trying to start a task that is a Divisory.
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
/// Match an Info task with its Informative task (if it is part of a Sequence).
///
/// If the task is an Info task, it will try to find its Informative task.
/// If the task is a Subtask, it will return its uuid.
/// If the task is a Sequence, but is not an Info task, it will try to find the next pending task.
///
/// # Errors
/// If there is no next task, it will return an error.
/// If the task is an Info task, but doesn't have an Informative task, it will return an error.
/// If the task is not a Sequence, it will return an error.
pub fn match_inforelat_and_sequence(
    filter_json: &TaskWarriorExported,
) -> Result<String, FypmError> {
    let state = &filter_json.state;

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
            let new_filter_json =
                get::json_by_filter(&inforelat, DEFAULT_GET_JSON_OPTIONS).unwrap();

            return match_inforelat_and_sequence(&new_filter_json[0]);
        } else {
            if is_sequence {
                if let Some(next_task) = &filter_json.seq_current {
                    let mut next_json = get::json_by_filter(&next_task, DEFAULT_GET_JSON_OPTIONS)?;
                    let mut status = next_json[0].status;

                    // Loop until find a pending task or there is no next task

                    while status == TaskWarriorStatus::Completed {
                        if let Some(next_task) = &next_json[0].seq_next {
                            next_json = get::json_by_filter(&next_task, DEFAULT_GET_JSON_OPTIONS)?;
                            status = next_json[0].status;
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
            if &filter_json.tags.is_some() == &true
                && &filter_json
                    .tags
                    .as_ref()
                    .unwrap()
                    .contains(&"SUBTASK".to_string())
                    == &true
            {
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
