//#region           External Imports
use crate::utils::structs;
use std::process::Command;
use std::str;
//#endregion

//#region           Enums
pub enum TimewAction {
    Start,
    End,
}
//#endregion
//#region           Functions
fn do_end_or_start(
    action: TimewAction,
    matches: &clap::ArgMatches,
    usage: &str,
) -> Result<(), String> {
    let received_args = matches.get_many::<String>("ACTIONARGS").unwrap();

    if received_args.len() == 1 {
        println!("{}", usage);
        Err(String::from("Do you not specify a task time?"))
    } else if received_args.len() == 0 {
        println!("{}", usage);
        Err(String::from("You need to specifiy something! >:("))
    } else if received_args.len() > 2 {
        println!("{}", usage);
        Err(String::from(
            "Hey... You don't need to specify more than two parameters... Is there an error here?",
        ))
    } else {
        Ok(timew_time_set(
            &action,
            received_args.clone().nth(0).unwrap(),
            received_args.clone().nth(1).unwrap(),
        ))
    }
}
fn do_end_or_start_correction(
    action: TimewAction,
    default_ids: Vec<&str>,
    matches: &clap::ArgMatches,
    usage: &str,
) -> Result<(), String> {
    let received_args = matches.get_many::<String>("ACTIONARGS").unwrap();

    let mut ids: Vec<&str> = vec![];

    for index in 0..=1 {
        if let Some(option_id) = received_args.clone().nth(index) {
            ids.push(option_id);
        } else {
            ids.push(default_ids[index]);
        }
    }

    if received_args.len() > 2 {
        println!("{}", usage);

        Err(String::from(
            "Hey... You don't need to specify more than two parameters!",
        ))
    } else {
        Ok(timew_time_move(&action, ids))
    }
}

pub fn timew_time_move(action: &TimewAction, ids: Vec<&str>) {
    fn receive_time(action: &TimewAction, id: &str) -> String {
        let get_task_info = Command::new("timew")
            .args([id, "export"])
            .output()
            .expect("Failed to get timew json!");

        let str_json = str::from_utf8(&get_task_info.stdout).unwrap();
        let tasks_json: Vec<structs::TimeWarriorExported> =
            serde_json::from_str(str_json).expect("Failed to parse received json!");

        let task_json = tasks_json.get(0).unwrap();

        if let TimewAction::Start = action {
            task_json.end.clone().expect("No end id provided!")
        } else {
            task_json.start.clone()
        }
    }

    timew_time_set(&action, ids[0], receive_time(&action, ids[1]).as_str());
}
pub fn timew_time_set(received_action: &TimewAction, received_id: &str, time: &str) {
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

    let execute = Command::new("timew")
        .args(["modify", &action, received_id, time, ":adjust"])
        .output();

    match execute {
        Ok(output) => {
            println!("{}", str::from_utf8(&output.stdout).unwrap());
        }
        Err(e) => eprintln!("Failed to execute timew command, error: {}", e),
    }
}
pub fn timew_track(matches: &clap::ArgMatches) -> Result<(), String> {
    let max_description_length = 25;

    let received_args = matches.get_many::<String>("ACTIONARGS").unwrap();
    let mut get_args = received_args.clone();

    let received_id = get_args.nth(0).expect("No task id provided!");

    let received_start_time = get_args.nth(0).expect("No task start time provided!");
    let receved_final_time = get_args.nth(0).expect("No task final time provided!");

    let get_task_info = Command::new("task")
        .args([received_id, "export"])
        .output()
        .expect("Failed to get task json!");

    let str_json = str::from_utf8(&get_task_info.stdout).unwrap();
    let tasks_json: Vec<structs::TaskWarriorExported> =
        serde_json::from_str(str_json).expect("Failed to parse received json!");

    let task_json = tasks_json.get(0).unwrap();

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
        received_start_time,
        "-",
        receved_final_time,
        &task_json.uuid[..8],
        &truncated_description,
        &task_json.wt,
        ":adjust",
    ];

    if let Some(style) = &task_json.style {
        args.push(style.as_str());
    }
    if let Some(r#type) = &task_json.r#type {
        args.push(r#type.as_str());
    }
    if let Some(project) = &task_json.project {
        args.push(project.as_str());
    }

    // Add tags
    if let Some(tags) = &task_json.tags {
        args.extend(tags.iter().map(|tag| tag.as_str()));
    }

    let execute = Command::new("timew").args(&args).output();

    match execute {
        Ok(output) => {
            println!("{}", str::from_utf8(&output.stdout).unwrap());
        }
        Err(e) => eprintln!("Failed to execute timew command, error: {}", e),
    }

    Ok(())
}
pub fn match_action(matches: &clap::ArgMatches) -> Result<(), String> {
    match matches.get_one::<String>("ACTION") {
        Some(action_value) => match action_value.as_str() {
            "end-correction" => {
                let usage = "Usage: timew end-correction [tomodify_task_end](default: @3) [static_task_start](default: @1)";
                let default_first_id = "@3";
                let default_second_id = "@1";
                let default_ids = vec![default_first_id, default_second_id];

                do_end_or_start_correction(TimewAction::End, default_ids, matches, usage)
            }
            "start-correction" => {
                let usage = "Usage: timew start-correction [tomodify_task_start](default: @1) [static_task_end](default: @3)";
                let default_first_id = "@1";
                let default_second_id = "@3";
                let default_ids = vec![default_first_id, default_second_id];

                do_end_or_start_correction(TimewAction::Start, default_ids, matches, usage)
            }
            "end" => {
                let usage = "Usage: timew end <task id> <task end time>";

                do_end_or_start(TimewAction::End, matches, usage)
            }
            "start" => {
                let usage = "Usage: timew start <task id> <task start time>";

                do_end_or_start(TimewAction::Start, matches, usage)
            }
            "track" => timew_track(matches),
            _ => panic!("No valid action provided!"),
        },
        None => {
            panic!("No argument provided!");
        }
    }
}
//#endregion
