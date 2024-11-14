//#region           External Imports
use chrono::{Duration, NaiveDate};
use colored::Colorize;
use std::process::Command;
use std::str::{self, FromStr};
use super::TimewAction;

//#endregion
//#region           Modules
use crate::utils::get;
use crate::func::matchs;

use fypm_lib::utils::date;
use fypm_lib::values::constants::DEFAULT_GET_JSON_OPTIONS;
use fypm_lib::values::err::{FypmError, FypmErrorKind};
//#endregion
//#region           Functions

/// Moves the start or end time of a timew log based on a reference log.
///
/// This function adjusts the specified timew log's start or end time by using either a reference log
/// or by calculating a new time based on the manipulation ID and the given action.
///
/// # Arguments
///
/// * `action` - The action to perform on the log. Use `TimewAction::Start` to adjust the start time
///   or `TimewAction::End` to adjust the end time.
/// * `manipulation_id` - A string starting with "@" followed by the ID number of the log to be manipulated.
/// * `reference_id` - An optional string starting with "@" that serves as the reference log for the time adjustment.
///
/// # Returns
///
/// * `Result<(), FypmError>` - Returns an `Ok` result if the log was successfully adjusted, or an `Err` result with
///   a `FypmError` if an error occurred.
///
/// # Panics
///
/// * Panics if the manipulation ID or the reference ID does not start with "@".
/// * Panics if the reference ID is omitted and the manipulation ID is less than 3 for an `End` action.
pub fn move_log(
    action: &TimewAction,
    manipulation_id: &String,
    reference_id: &Option<String>,
) -> Result<(), FypmError> {
    let id_err = "Hey!! Are you trying to use a taskwarrior id? Specify with \"@\"!";

    let inverted_action: &TimewAction;

    match action {
        TimewAction::Start => {
            inverted_action = &TimewAction::End;
        }
        TimewAction::End => {
            inverted_action = &TimewAction::Start;
        }
    }

    let time: String;

    if let Some(id) = reference_id {
        if !manipulation_id.starts_with("@") || !id.starts_with("@") {
            panic!("{}", id_err);
        }

        time = get::get_timew_time(id, &inverted_action);
    } else {
        if !manipulation_id.starts_with("@") {
            panic!("{}", id_err);
        }

        let id_number = manipulation_id
            .trim_start_matches("@")
            .parse::<usize>()
            .unwrap();
        let final_number: usize;

        if let TimewAction::Start = action {
            final_number = id_number + 2;
        } else {
            if id_number >= 3 {
                final_number = id_number - 2;
            } else {
                panic!("You're trying to omit the second id, but the first id is less than 3!");
            }
        }

        time = get::get_timew_time(&format!("@{}", final_number), &inverted_action);
    }

    set_log(&action, manipulation_id, &time)
}
/// Modifies the start or end time of a timew log.
///
/// # Arguments
///
/// * `received_action` - The action to be performed. Use `TimewAction::Start` to set the start time or `TimewAction::End` to set the end time.
/// * `received_id` - The id of the log. Use a string starting with "@", followed by the id number.
/// * `received_time` - The time to be set. If the string starts with "@", it will use the special timing properties (see `matchs::match_special_timing_properties`).
///
/// # Returns
///
/// * `Result` - If the command finished successfully, it will return an `Ok` result. If an error occurred, it will return an `Err` result with a `FypmError` containing the error message.
pub fn set_log(
    received_action: &TimewAction,
    received_id: &String,
    received_time: &String,
) -> Result<(), FypmError> {
    if !received_id.starts_with("@") {
        panic!("Hey!! Are you trying to use a taskwarrior id? Specify with \"@\"!");
    }

    let mut action = String::new();
    match received_action {
        TimewAction::Start => {
            action.push_str("start");
        }
        TimewAction::End => {
            action.push_str("end");
        }
    }
    let mut time: String = received_time.to_string();

    if time.starts_with("@") {
        time = matchs::match_special_timing_properties(received_time).unwrap();
    }

    let execute = Command::new("timew")
        .args(["modify", &action, received_id, &time, ":adjust"])
        .output();

    match execute {
        Ok(output) => {
            if output.status.success() {
                println!("{}", str::from_utf8(&output.stdout).unwrap());
            } else {
                eprintln!("{}", str::from_utf8(&output.stderr).unwrap());
            }

            Ok(())
        }
        Err(e) => panic!("Failed to execute timew command, error: {}", e),
    }
}
/// Track time entries for a taskwarrior task.
///
/// This function processes the given time intervals and assigns them to the specified
/// taskwarrior task. It supports special timing properties and aliases for IDs.
///
/// # Arguments
///
/// * `received_id` - A reference to a string representing the task ID.
/// * `params` - A vector of strings representing pairs of start and end times.
///
/// # Returns
///
/// * `Result<(), FypmError>` - Returns an Ok result if successful, or a `FypmError` if
///   an error occurs during time tracking.
///
/// # Errors
///
/// * `FypmErrorKind::InvalidInput` - If an invalid number of parameters is provided.
pub fn track(received_id: &String, params: &Vec<String>) -> Result<(), FypmError> {
    let id = matchs::match_special_aliases(received_id);

    if params.len() % 2 != 0 {
        return Err(FypmError {
            message:
                "Invalid number of parameters! You have to specify an even number of parameters!"
                    .to_string(),
            kind: FypmErrorKind::InvalidInput,
        });
    }

    for i in (0..params.len()).step_by(2) {
        let cur_start = params[i].clone().to_string();
        let cur_end = params[i + 1].clone().to_string();

        let start_time = if cur_start.starts_with("@") {
            matchs::match_special_timing_properties(&cur_start).unwrap()
        } else {
            cur_start
        };

        let end_time = if cur_end.starts_with("@") {
            matchs::match_special_timing_properties(&cur_end).unwrap()
        } else {
            cur_end
        };

        if received_id.starts_with("@") {
            let execute = Command::new("timew")
                .args([
                    &String::from("continue"),
                    &received_id,
                    &start_time,
                    &String::from("-"),
                    &end_time,
                    &String::from(":adjust"),
                ])
                .output();

            match execute {
                Ok(output) => {
                    if output.status.success() {
                        println!("{}", str::from_utf8(&output.stdout).unwrap());
                    } else {
                        eprintln!("{}", str::from_utf8(&output.stderr).unwrap());
                    }
                }
                Err(e) => eprintln!("Failed to execute timew command, error: {}", e),
            }
        } else {
            let max_description_length = 25;

            let get_task_json = get::json_by_filter(&id, DEFAULT_GET_JSON_OPTIONS).unwrap();
            let task_json = get_task_json.get(0).unwrap();

            let mut truncated_description = String::new();
            if &task_json.description.len() > &25 {
                truncated_description = format!(
                    "{}...",
                    &task_json.description[..max_description_length - 3]
                )
            } else {
                truncated_description.push_str(&task_json.description);
            }

            let mut args = vec![
                "track",
                &start_time,
                "-",
                &end_time,
                &task_json.uuid,
                &truncated_description,
                &task_json.wt,
                ":adjust",
                &task_json.r#type,
            ];

            if let Some(style) = &task_json.style {
                args.push(&style);
            }

            if let Some(project) = &task_json.project {
                args.push(&project);
            }

            // Add tags
            if let Some(tags) = &task_json.tags {
                args.extend(tags.iter().map(|tag| tag.as_str()));
            }

            let execute = Command::new("timew").args(&args).output();

            match execute {
                Ok(output) => {
                    if output.status.success() {
                        println!("{}", str::from_utf8(&output.stdout).unwrap());
                    } else {
                        eprintln!("{}", str::from_utf8(&output.stderr).unwrap());
                    }
                }
                Err(e) => eprintln!("Failed to execute timew command, error: {}", e),
            }
        }
    }

    Ok(())
}
pub fn replace(
    received_original_id: &String,
    received_replacement_id: &String,
) -> Result<(), FypmError> {
    if !received_original_id.starts_with("@") {
        panic!("Hey!! The second argument should be a timewarrior id! Specify with \"@\"!");
    }

    let start_time = get::get_timew_time(received_original_id, &TimewAction::Start);
    let end_time = get::get_timew_time(received_original_id, &TimewAction::End);

    track(received_replacement_id, &vec![start_time, end_time])
}
/// Lists timewarrior entries between the given initial and final dates
///
/// If the final date is not provided, it will be considered as the initial date plus one day.
///
/// # Arguments
///
/// * `initial_date` - A string representing the initial date for the range to be listed.
///                    It should be in a format that can be parsed by the `date` crate.
/// * `final_date` - An optional string representing the final date for the range to be listed.
///                  It should be in a format that can be parsed by the `date` crate.
///
/// # Errors
///
/// * `FypmError` - If the dates cannot be parsed, or if there is an error while retrieving the
///                timewarrior entries or tasks.
pub fn list(initial_date: &String, final_date: &Option<String>) -> Result<(), FypmError> {
    let start: NaiveDate;
    let end: NaiveDate;

    start = NaiveDate::from_str(&date::match_aliases(initial_date)).unwrap();

    if let Some(final_date) = final_date {
        end = NaiveDate::from_str(&date::match_aliases(final_date)).unwrap();
    } else {
        end = start + Duration::days(1);
    }

    let (timew_entries, tasks_map) = get::timew_entries(start, end)?;

    for entry in timew_entries {
        println!(
            "{} - {}",
            format!("@{}", entry.1.id).to_string().bold().truecolor(180, 0, 230),
            tasks_map
                .get(&entry.0)
                .expect("There is a problem with the UUID!")
                .description
        );
    }

    Ok(())
}
//#endregion
