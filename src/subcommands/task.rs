//#region           Crates
use std::{fs, process::Command};
//#endregion
//#region           Modules
use crate::func::action::*;
use crate::func::parser;
use crate::utils::constants::{CONTROL_TASK, DEFAULT_GET_JSON_OPTIONS, LAST_TASK_PATH};
use crate::utils::err::FypmErrorKind;
use crate::utils::get;
//#endregion
//#region           Implementation

pub fn task_stop(filter_option: &Option<String>, start_control_task: bool) {
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
        task_start(&CONTROL_TASK.to_string());
    }
}
pub fn task_start(filter: &String) {
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
                FypmErrorKind::TooMuchArgs => {
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
            task_stop(&Some(active_task_uuid.to_string()), false);
        }

        println!("Starting task with uuid: {}", filter);
        Command::new("task")
            .args([filter.as_str(), "start"])
            .output()
            .unwrap();
    }
}
pub fn task_done(filter: &Option<String>, tastart_filter: &Option<String>) {
    if let Some(filter) = filter {
        let task_json = get::get_json_by_filter(filter, None).unwrap();

        if let Some(tastart_filter) = tastart_filter {
            task_start(tastart_filter);
        } else {
            let current_task = get::get_current_task_json().unwrap();

            for task in &task_json {
                if task.uuid == current_task.uuid {
                    task_start(&CONTROL_TASK.to_string());
                    break;
                }
            }
        }

        Command::new("task")
            .args([filter, "done"])
            .output()
            .unwrap();
    } else {
        let current_task = get::get_current_task_json().unwrap();

        if let Some(tastart_filter) = tastart_filter {
            task_start(tastart_filter);
        } else {
            task_start(&CONTROL_TASK.to_string());
        }

        Command::new("task")
            .args([current_task.uuid.as_str(), "done"])
            .output()
            .unwrap();
    }
}
//#endregion
