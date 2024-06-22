//#region           External Imports
use chrono::{Duration, NaiveDate};
use std::process::{Command, Stdio};
use std::str::{self, FromStr};

//#endregion
//#region           Modules
use crate::func::{date, parser};
use crate::utils::constants::DEFAULT_GET_JSON_OPTIONS;
use crate::utils::enums::TimewAction;
use crate::utils::err::FypmError;
use crate::utils::get;
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
pub fn track(
    received_id: &String,
    received_start_time: &String,
    received_end_time: &String,
) -> Result<(), FypmError> {
    let id = parser::match_special_aliases(received_id);

    let mut start_time = received_start_time.to_string();
    let mut end_time = received_end_time.to_string();

    {
        if received_start_time.starts_with("@") {
            start_time = parser::match_special_timing_properties(received_start_time).unwrap();
        }
        if received_end_time.starts_with("@") {
            end_time = parser::match_special_timing_properties(received_end_time).unwrap();
        }
    }

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

        let get_task_json = get::get_json_by_filter(&id, DEFAULT_GET_JSON_OPTIONS).unwrap();
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

    track(received_replacement_id, &start_time, &end_time)
}
pub fn list(date: &String, aditional_filters: &Option<Vec<String>>) -> Result<(), FypmError> {
    let initial_date = NaiveDate::from_str(&date::match_aliases(date)).unwrap();
    let final_date = initial_date + Duration::days(1);

    let mut args = Vec::new();

    args.extend([
        "summary".to_string(),
        ":ids".to_string(),
        initial_date.to_string(),
        "-".to_string(),
        final_date.to_string(),
    ]);
    if let Some(filters) = aditional_filters {
        args.extend(filters.clone())
    }

    Command::new("timew")
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .output()
        .unwrap();

    Ok(())
}
//#endregion
