use std::io::Error;
//#region           External Imports
use std::process::Command;
use std::str;
//#endregion
//#region           Modules
use crate::func::actions;
use crate::utils::structs;
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
    actionargs: &Vec<String>,
    usage: &str,
) -> Result<(), Error> {
    if actionargs.len() != 2 {
        println!("{}", usage);

        Err(Error::new(
            std::io::ErrorKind::InvalidInput,
            String::from("Hey... You don't need to specify more than two parameters!"),
        ))
    } else {
        let received_id = actionargs.get(0).unwrap().to_string();
        let time = actionargs.get(1).unwrap().to_string();

        timew_time_set(&action, received_id, time)
    }
}
fn do_end_or_start_correction(
    action: TimewAction,
    actionargs: &Vec<String>,
    default_ids: [&str; 2],
    usage: &str,
) -> Result<(), Error> {
    if actionargs.len() > 2 {
        println!("{}", usage);

        Err(Error::new(
            std::io::ErrorKind::InvalidInput,
            String::from("Hey... You don't need to specify more than two parameters!"),
        ))
    } else {
        let mut ids: Vec<String> = Vec::new();

        for index in 0..=1 {
            if let Some(received_id) = actionargs.get(index) {
                ids.push(received_id.to_string());
            } else {
                ids.push(default_ids[index].to_string());
            }
        }

        timew_time_move(&action, ids)
    }
}

pub fn timew_time_move(action: &TimewAction, ids: Vec<String>) -> Result<(), Error> {
    fn receive_time(action: &TimewAction, id: String) -> String {
        let get_task_info = Command::new("timew")
            .args([id, String::from("export")])
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

    let received_id = ids.get(0).unwrap().to_string();
    let time = receive_time(&action, ids.get(1).unwrap().to_string());

    timew_time_set(&action, received_id, time)
}
pub fn timew_time_set(
    received_action: &TimewAction,
    received_id: String,
    time: String,
) -> Result<(), Error> {
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
        .args([
            String::from("modify"),
            action,
            received_id,
            time,
            String::from(":adjust"),
        ])
        .output();

    match execute {
        Ok(output) => {
            println!("{}", str::from_utf8(&output.stdout).unwrap());

            Ok(())
        }
        Err(e) => Err(e),
    }
}
pub fn timew_track(
    received_id: &String,
    received_start_time: &String,
    receved_final_time: &String,
) -> Result<(), Error> {
    let get_task_info = Command::new("task")
        .args([&received_id, "export"])
        .output()
        .expect("Failed to get task json!");

    if received_id.starts_with("@") {
        let execute = Command::new("timew")
            .args([
                &String::from("continue"),
                received_id,
                received_start_time,
                &String::from("-"),
                receved_final_time,
                &String::from(":adjust"),
            ])
            .output();

        match execute {
            Ok(output) => {
                println!("{}", str::from_utf8(&output.stdout).unwrap());
            }
            Err(e) => eprintln!("Failed to execute timew command, error: {}", e),
        }
    } else {
        let max_description_length = 25;
        let str_json = str::from_utf8(&get_task_info.stdout).unwrap();
        let tasks_json = serde_json::from_str::<Vec<structs::TaskWarriorExported>>(str_json)
            .expect("Failed to parse received json!");

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
            &task_json.r#type
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
                println!("{}", str::from_utf8(&output.stdout).unwrap());
            }
            Err(e) => eprintln!("Failed to execute timew command, error: {}", e),
        }
    }

    Ok(())
}

pub fn match_action(action: &String, actionargs: &Vec<String>) -> Result<(), Error> {
    match action.as_str() {
        "end-correction" => {
            let usage = "Usage: timew end-correction [tomodify_task_end](default: @3) [static_task_start](default: @1)";
            let default_first_id = "@3";
            let default_second_id = "@1";
            let default_ids = [default_first_id, default_second_id];

            do_end_or_start_correction(TimewAction::End, actionargs, default_ids, usage)
        }
        "start-correction" => {
            let usage = "Usage: timew start-correction [tomodify_task_start](default: @1) [static_task_end](default: @3)";
            let default_first_id = "@1";
            let default_second_id = "@3";
            let default_ids = [default_first_id, default_second_id];

            do_end_or_start_correction(TimewAction::Start, actionargs, default_ids, usage)
        }
        "end" => {
            let usage = "Usage: timew end <task id> <task end time>";

            do_end_or_start(TimewAction::End, actionargs, usage)
        }
        "start" => {
            let usage = "Usage: timew start <task id> <task start time>";

            do_end_or_start(TimewAction::Start, actionargs, usage)
        }
        "track" => {
            if actionargs.len() != 3 {
                panic!("Not enough arguments provided!");
            }

            let received_id = &actionargs[0];
            let received_start_time = &actionargs[1];
            let receved_final_time = &actionargs[2];

            timew_track(received_id, received_start_time, receved_final_time)
        }
        "annotate" => {
            if actionargs.len() != 2 {
                panic!("Not enough arguments provided!");
            }

            let id = &actionargs[0];
            let annotation = &actionargs[1];

            actions::annotate("timew", id, annotation)
        }
        _ => panic!("No valid action provided!"),
    }
}
//#endregion
