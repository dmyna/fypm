//#region           Crates
use dialoguer::Confirm;
use regex::Regex;
use std::{fs, process::Command, str};

//#endregion
//#region           Modules
use crate::func::{action::*, list, parser};
use crate::utils::constants::{CONTROL_TASK, DEFAULT_GET_JSON_OPTIONS, LAST_TASK_PATH};
use crate::utils::enums;
use crate::utils::err::FypmError;
use crate::utils::err::FypmErrorKind;
use crate::utils::get;
//#endregion
//#region           Implementation

pub fn task_stop(
    filter_option: &Option<String>,
    start_control_task: bool,
) -> Result<(), FypmError> {
    let final_filter: String;

    if let Some(filter) = filter_option {
        final_filter = filter.to_string();
    } else {
        let active_tasks = get::get_current_task_json().unwrap();

        final_filter = active_tasks.uuid.to_string();
    }

    Command::new("task")
        .args([&final_filter, "stop"])
        .output()
        .unwrap();

    if start_control_task {
        task_start(&CONTROL_TASK.to_string())?;
    }

    Ok(())
}
pub fn task_start(filter: &String) -> Result<(), FypmError> {
    let mut filter = parser::match_special_aliases(filter);
    let filter_json = get::get_json_by_filter(&filter, DEFAULT_GET_JSON_OPTIONS).unwrap();
    let filter_length = filter_json.len();

    if filter_length == 0 {
        panic!("No task with filter \"{}\" found!", filter);
    } else if filter_length > 1 {
        panic!("Too much tasks with filter \"{}\"!", filter);
    }

    verify_if_wt_is_allday(&filter_json[0]).unwrap();

    verify_if_is_divisory(&filter_json[0]).unwrap();

    filter = match_inforelat_and_sequence(&filter_json[0]).unwrap();

    {
        // !DEV: Implement tascripts in Rust later

        Command::new("tascripts").args([&filter]).output().unwrap();
    }

    {
        let active_tasks = get::get_current_task_json();

        if active_tasks.is_err() {
            let err = active_tasks.unwrap_err();

            match err.kind {
                FypmErrorKind::TooMuchTasks => {
                    panic!("There are more than one task active! Fix it >:(.");
                }
                FypmErrorKind::NoTasksFound => {}
                e => {
                    panic!("Unexpected error: {:?}", e);
                }
            }
        } else {
            let active_task_uuid = &active_tasks.unwrap().uuid;
            fs::write(LAST_TASK_PATH, active_task_uuid.as_bytes()).unwrap();

            println!("Stopping active task with uuid: {}", active_task_uuid);
            task_stop(&Some(active_task_uuid.to_string()), false).unwrap();
        }

        println!("Starting task with uuid: {}", filter);
        Command::new("task")
            .args([filter.as_str(), "start"])
            .output()
            .unwrap();

        Ok(())
    }
}
pub fn task_done(
    filter: &Option<String>,
    tastart_filter: &Option<String>,
) -> Result<(), FypmError> {
    if let Some(filter) = filter {
        let task_json = get::get_json_by_filter(filter, None)?;

        if let Some(tastart_filter) = tastart_filter {
            task_start(tastart_filter)?;
        } else {
            let current_task = get::get_current_task_json()?;

            for task in &task_json {
                if task.uuid == current_task.uuid {
                    task_start(&CONTROL_TASK.to_string())?;
                    break;
                }
            }
        }

        Command::new("task")
            .args([filter, "done"])
            .output()
            .unwrap();
    } else {
        let current_task = get::get_current_task_json()?;

        if let Some(tastart_filter) = tastart_filter {
            task_start(tastart_filter)?;
        } else {
            task_start(&CONTROL_TASK.to_string())?;
        }

        Command::new("task")
            .args([current_task.uuid.as_str(), "done"])
            .output()
            .unwrap();
    }

    Ok(())
}
pub fn task_statistic(
    command: &enums::StatisticsCommands,
    no_parents: &bool,
) -> Result<(), FypmError> {
    match command {
        enums::StatisticsCommands::Deleted => {
            list::deleted_tasks(no_parents)?;
        }
        enums::StatisticsCommands::Pending => {
            list::pending_tasks(no_parents)?;
        }
    }

    Ok(())
}
pub fn task_add(
    description: &String,
    project: &String,
    style: &String,
    r#type: &String,
    other_args: &Option<Vec<String>>,
    skip_confirmation: &bool,
    return_uuid: &bool,
) -> Result<enums::TaskAddReturn, FypmError> {
    if !*skip_confirmation {
        println!("These are the args:");
        println!("      description: {}", description);
        println!("      project: {}", project);
        println!("      STYLE: {}", style);
        println!("      TYPE: {}, ", r#type);
        println!(
            "      others: {}",
            other_args.as_ref().unwrap_or(&vec![]).join(" ")
        );

        let confirmation = Confirm::new()
            .with_prompt("Do you want to continue?")
            .interact()
            .unwrap();

        if !confirmation {
            return Ok(enums::TaskAddReturn::Default(()));
        }
    }

    let mut args = vec![
        "rc.verbose=new-uuid".to_string(),
        "add".to_string(),
        description.to_string(),
        format!("project:{}", project),
        format!("STYLE:{}", style),
        format!("TYPE:{}", r#type),
    ];

    if let Some(other_args) = other_args {
        args.extend(other_args.clone());
    }

    let execute = Command::new("task").args(args).output();

    let uuid: String;
    {
        let regex = Regex::new(
            r"[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}",
        )
        .unwrap();

        if let Ok(output) = execute {
            if output.status.success() {
                let stdout = str::from_utf8(&output.stdout).unwrap();

                if let Some(captured) = regex.captures(stdout) {
                    uuid = captured[0].to_string();
                } else {
                    println!("No created tasks!");
                    panic!("{}", stdout)
                }
            } else {
                panic!(
                    "An error occurred trying to create a task: {}",
                    str::from_utf8(&output.stderr).unwrap()
                );
            }
        } else {
            let error = execute.unwrap_err();

            panic!("An error occurred trying to create a task: {}", error);
        }
    }

    println!("Created task with uuid: {}!", uuid);

    if *return_uuid {
        Ok(enums::TaskAddReturn::UUID(uuid))
    } else {
        Ok(enums::TaskAddReturn::Default(()))
    }
}
pub fn task_add_sub(
    mother_task: &String,
    other_args: &Vec<String>,
    skip_confirmation: &bool,
) -> Result<(), FypmError> {
    let subtask: &String;

    let get_mother_task_json = get::get_json_by_filter(mother_task, DEFAULT_GET_JSON_OPTIONS)?;
    let mother_task_json = get_mother_task_json.get(0).unwrap();

    if other_args.len() == 1 {
        subtask = other_args.get(0).unwrap();

        let get_subtask_uuid = get::get_uuids_by_filter(subtask, DEFAULT_GET_JSON_OPTIONS)?;
        let subtask_uuid = get_subtask_uuid.get(0).unwrap();

        Command::new("task")
            .args([subtask_uuid.as_str(), "modify", "TYPE:SubTask"])
            .output()
            .unwrap();
    } else if other_args.len() >= 2 {
        let project: &String;

        if let Some(project_arg) = &mother_task_json.project {
            project = project_arg;
        } else {
            panic!("The specified mother doesn't have a project setted... Are you writing this stuff right?");
        }

        let create_task = task_add(
            other_args.get(0).unwrap(),
            project,
            other_args.get(1).unwrap(),
            &"SubTask".to_string(),
            &other_args.get(2..).map(|x| x.to_vec()),
            skip_confirmation,
            &true,
        )?;

        match create_task {
            enums::TaskAddReturn::UUID(_) => subtask = other_args.get(0).unwrap(),
            enums::TaskAddReturn::Default(_) => {
                panic!("task_add is returning void with return_uuid setted as true!")
            }
        }
    } else {
        panic!("You specified a wrong number of arguments! You don't know how to read documentation, do you? :P");
    }

    Command::new("task")
        .args([mother_task.as_str(), "modify", "STATE:Info", "+MOTHER"])
        .output()
        .unwrap();
    println!("Mother task setted.");

    Command::new("task")
        .args([
            subtask,
            &"modify".to_string(),
            &format!("MOTHER:{}", mother_task_json.uuid),
        ])
        .output()
        .unwrap();
    println!(
        "Subtask added to its MOTHER '{}'!",
        mother_task_json.description
    );

    Ok(())
}
//#endregion
