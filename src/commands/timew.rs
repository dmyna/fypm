//#region           External Imports
use chrono::{Duration, NaiveDate};
use colored::Colorize;
use std::collections::HashMap;
use std::process::{Command, Stdio};
use std::str::{self, FromStr};
use uuid::Uuid;

//#endregion
//#region           Modules
use crate::func::{date, parser};
use crate::utils::get;
use crate::values::constants::DEFAULT_GET_JSON_OPTIONS;
use crate::values::enums::TimewAction;
use crate::values::err::{FypmError, FypmErrorKind};
use crate::values::structs::{TaskWarriorExported, TimeWarriorExported};
//#endregion
//#region           Functions
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
        time = parser::match_special_timing_properties(received_time).unwrap();
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
pub fn track(received_id: &String, params: &Vec<String>) -> Result<(), FypmError> {
    let id = parser::match_special_aliases(received_id);

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
            parser::match_special_timing_properties(&cur_start).unwrap()
        } else {
            cur_start
        };

        let end_time = if cur_end.starts_with("@") {
            parser::match_special_timing_properties(&cur_end).unwrap()
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
pub fn list(initial_date: &String, final_date: &Option<String>) -> Result<(), FypmError> {
    let start: NaiveDate;
    let end: NaiveDate;

    start = NaiveDate::from_str(&date::match_aliases(initial_date)).unwrap();

    if let Some(final_date) = final_date {
        end = NaiveDate::from_str(&date::match_aliases(final_date)).unwrap();
    } else {
        end = start + Duration::days(1);
    }

    let timew_json =
        get::get_timew_json_by_filter(&vec![start.to_string(), "-".to_string(), end.to_string()])
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

    let tasks_json = get::json_by_filter(
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
