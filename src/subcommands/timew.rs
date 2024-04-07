use std::io::Error;
//#region           External Imports
use std::process::Command;
use std::str;
//#endregion
//#region           Modules
use crate::utils::structs;
//#endregion
//#region           Enums
pub enum TimewAction {
    Start,
    End,
}
//#endregion
//#region           Functions
pub fn time_move(action: &TimewAction, ids: Vec<&String>) -> Result<(), Error> {
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

    let received_id = ids.get(0).unwrap();
    let time = receive_time(&action, ids.get(1).unwrap().to_string());

    time_set(&action, *received_id, &time)
}
pub fn time_set(
    received_action: &TimewAction,
    received_id: &String,
    time: &String,
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
            "modify",
            &action,
            received_id,
            time,
            ":adjust",
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
pub fn track(
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
//#endregion
