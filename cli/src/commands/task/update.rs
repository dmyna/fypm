use std::io::Write;
use std::{
    fs,
    process::{Command, Stdio},
};

use chrono::NaiveTime;
use dialoguer::Input;

use fypm_lib::utils::parser;
use fypm_lib::values::{
    constants::{CONTROL_TASK, DEFAULT_GET_JSON_OPTIONS, LAST_TASK_PATH},
    err::{FypmError, FypmErrorKind},
    structs::{TaskWarriorExported, TaskWarriorStatus},
};

use crate::func::matchs;
use crate::{
    commands,
    func::{
        action::{
            self, match_inforelat_and_sequence, verify_if_is_divisory, verify_if_wt_is_allday,
        },
        command, dialog,
    },
    utils::get,
};

pub fn stop(filter_option: &Option<String>, start_control_task: bool) -> Result<(), FypmError> {
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
        start(&CONTROL_TASK.to_string())?;
    }

    Ok(())
}
pub fn start(filter: &String) -> Result<(), FypmError> {
    let mut filter = matchs::match_special_aliases(filter);
    let filter_json = if filter.starts_with("+ST_") {
        get::mother_json_by_sequence_id(&filter)?
    } else {
        get::json_by_filter(&filter, DEFAULT_GET_JSON_OPTIONS)?
            .get(0)
            .unwrap()
            .clone()
    };

    verify_if_wt_is_allday(&filter_json).unwrap();

    verify_if_is_divisory(&filter_json).unwrap();

    filter = match_inforelat_and_sequence(&filter_json).unwrap();

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
            stop(&Some(active_task_uuid.to_string()), false).unwrap();
        }

        println!("Starting task with uuid: {}", filter);
        Command::new("task")
            .args([filter.as_str(), "start"])
            .output()
            .unwrap();

        #[cfg(feature = "fysm")]
        {
            //. DEV: Implement tascripts in Rust later

            Command::new("tascripts").args([&filter]).output().unwrap();
        }

        Ok(())
    }
}
pub fn done(
    filter: &Option<String>,
    tastart_filter: &Option<String>,
    annotation: &Option<String>,
    skip_confirmation: &bool,
    not_necessary: &bool,
    delegated: &bool,
) -> Result<(), FypmError> {
    let mut args = vec!["rc.confirmation=0", "rc.recurrence.confirmation=0"];
    let selected_tasks: Vec<TaskWarriorExported>;

    if let Some(filter) = filter {
        let task_json = get::json_by_filter(filter, None)?;

        if let Some(tastart_filter) = tastart_filter {
            start(tastart_filter)?;
        } else {
            let current_task = get::get_current_task_json()?;

            for task in &task_json {
                if task.uuid == current_task.uuid {
                    start(&CONTROL_TASK.to_string())?;
                    break;
                }
            }
        }

        selected_tasks = task_json;
    } else {
        let current_task = get::get_current_task_json()?;

        if let Some(tastart_filter) = tastart_filter {
            start(tastart_filter)?;
        } else {
            start(&CONTROL_TASK.to_string())?;
        }

        selected_tasks = vec![current_task];
    }

    let join_uuids = selected_tasks
        .iter()
        .map(|task| task.uuid.as_str())
        .collect::<Vec<&str>>()
        .join(" ");

    args.extend([join_uuids.as_str()]);

    let confirmation: bool;

    if *skip_confirmation {
        confirmation = true;
    } else {
        confirmation = dialog::verify_selected_tasks(&selected_tasks)?;
    }

    if confirmation {
        if let Some(annotation) = annotation {
            action::annotate("task", &join_uuids, annotation, true)?;
        }

        // Tags logic
        {
            let mut tags: Vec<&str> = vec![];

            if *not_necessary {
                tags.push("+NotNecessary");
            }

            if *delegated {
                tags.push("+Delegated");
            }

            if tags.len() == 2 {
                return Err(FypmError {
                    message: "You are trying to mark a task with two tags! Are you crazy?"
                        .to_string(),
                    kind: FypmErrorKind::InvalidInput,
                });
            } else if tags.len() == 1 {
                let mut tags_args = args.clone();

                tags_args.push("modify");
                tags_args.extend(tags);

                let mut tag_binding = Command::new("task");
                let tag_command = tag_binding
                    .args(tags_args)
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit());

                let mut tag_child = tag_command.stdin(Stdio::piped()).spawn().unwrap();

                tag_child
                    .stdin
                    .take()
                    .unwrap()
                    .write_all("all\n".as_bytes())
                    .unwrap();
                tag_child.wait().unwrap();
            }
        }
        let mut done_args = args.clone();

        done_args.push("done");

        let mut done_binding = Command::new("task");
        let done_command = done_binding
            .args(done_args)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit());

        if selected_tasks.len() > 2 {
            let mut done_child = done_command.stdin(Stdio::piped()).spawn().unwrap();

            done_child
                .stdin
                .take()
                .unwrap()
                .write_all("all\n".as_bytes())
                .unwrap();
            done_child.wait().unwrap();
        } else {
            done_command.output().unwrap();
        }
    } else {
        println!("Aborting...");
    }

    Ok(())
}

pub fn abandon(
    tag: &commands::TaAbandonTags,
    filter: &String,
    annotation: &Option<String>,
    annotation_filter: &Option<String>,
) -> Result<(), FypmError> {
    if (tag == &commands::TaAbandonTags::Abandoned || tag == &commands::TaAbandonTags::NoControl)
        && annotation.is_none()
    {
        panic!("You must specify an annotation when mark a task as NoControl or Abandoned!");
    }
    let tasks = get::json_by_filter(filter, None)?;
    let tasks_count: usize = tasks.len();
    let confirmation = dialog::verify_selected_tasks(&tasks)?;

    if confirmation {
        let mut modify_args = Vec::new();
        modify_args.extend([
            "rc.verbose=0".to_string(),
            "rc.recurrence.confirmation=0".to_string(),
            "rc.confirmation=0".to_string(),
            filter.clone(),
            "modify".to_string(),
        ]);

        match tag {
            commands::TaAbandonTags::Archived => {
                modify_args.extend(["+Archived".to_string()]);
            }
            commands::TaAbandonTags::Failed => {
                modify_args.extend(["+Failed".to_string()]);
            }
            commands::TaAbandonTags::Abandoned => {
                modify_args.extend(["+Abandoned".to_string()]);
            }
            commands::TaAbandonTags::Chain => {
                modify_args.extend(["+Chain".to_string()]);

                let chain_task = Input::<String>::new()
                    .with_prompt("Specify the chain task that triggered this failure")
                    .validate_with(|input: &String| {
                        if input.trim().is_empty() {
                            Err("You must specify a chain task!".to_string())
                        } else {
                            let task = get::json_by_filter(input, DEFAULT_GET_JSON_OPTIONS);

                            if task.is_ok() {
                                Ok(())
                            } else {
                                let err = task.unwrap_err();
                                if err.kind == FypmErrorKind::NoTasksFound {
                                    Err("This chain task does not exist!".to_string())
                                } else {
                                    Err(err.message)
                                }
                            }
                        }
                    })
                    .interact_text()
                    .unwrap();

                let chain_uuid = get::get_uuids_by_filter(&chain_task, DEFAULT_GET_JSON_OPTIONS)
                    .unwrap()
                    .get(0)
                    .unwrap()
                    .clone();

                modify_args.extend([format!("CHAIN:{}", chain_uuid)]);
            }
            commands::TaAbandonTags::NoControl => {
                modify_args.extend(["+NoControl".to_string()]);
            }
        }

        if let Some(annotation) = annotation {
            if let Some(annotation_filter) = annotation_filter {
                action::annotate("task", annotation_filter, annotation, true)?;
            } else {
                action::annotate("task", filter, annotation, true)?;
            }
        }

        let mut modify_binding = Command::new("task");
        let modify_command = modify_binding.args(modify_args).stderr(Stdio::inherit());

        let mut delete_binding = Command::new("task");
        let delete_command = delete_binding
            .args([
                "rc.verbose=0",
                "rc.confirmation=0",
                "rc.recurrence.confirmation=0",
                filter,
                "delete",
            ])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit());

        if tasks_count > 2 {
            command::stdin_all(modify_command).unwrap();

            command::stdin_all(delete_command).unwrap();
        } else {
            modify_command.output().unwrap();

            delete_command.output().unwrap();
        }
    } else {
        println!("Aborting...");
    }

    Ok(())
}
pub fn schedule(
    filter: &String,
    alarm_date: &String,
    due_date: &Option<String>,
    worktime: &Option<String>,
) -> Result<(), FypmError> {
    let tasks = get::json_by_filter(filter, None)?;
    let tasks_count: usize = tasks.len();
    let confirmation = dialog::verify_selected_tasks(&tasks)?;

    if confirmation {
        let mut modify_args = Vec::new();
        modify_args.extend([
            "rc.verbose=0".to_string(),
            "rc.recurrence.confirmation=0".to_string(),
            "rc.confirmation=0".to_string(),
            filter.clone(),
            "modify".to_string(),
        ]);

        if alarm_date != "cur" {
            modify_args.extend([format!("ALARM:{}", alarm_date)]);
            modify_args.extend([format!("scheduled:{}", alarm_date)]);
        }
        if let Some(due_date) = due_date {
            if due_date != "cur" {
                modify_args.extend([format!("due:{}", due_date)]);
            }
        }
        if let Some(worktime) = worktime {
            modify_args.extend([format!("WT:{}", worktime)]);
        }

        let mut modify_binding = Command::new("task");
        let modify_command = modify_binding
            .args(modify_args)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit());

        if tasks_count > 2 {
            let mut modify_child = modify_command.stdin(Stdio::piped()).spawn().unwrap();

            modify_child
                .stdin
                .take()
                .unwrap()
                .write_all("all\n".as_bytes())
                .unwrap();
            modify_child.wait().unwrap();
        } else {
            modify_command.output().unwrap();
        }
    } else {
        println!("Aborting...");
    }

    Ok(())
}
pub fn unschedule(
    filter: &String,
    no_alarm: &bool,
    no_due: &bool,
    no_worktime: &bool,
) -> Result<(), FypmError> {
    let tasks = get::json_by_filter(filter, None)?;
    let tasks_count: usize = tasks.len();
    let confirmation = dialog::verify_selected_tasks(&tasks)?;

    if confirmation {
        let mut modify_args = Vec::new();
        modify_args.extend([
            "rc.verbose=0",
            "rc.recurrence.confirmation=0",
            "rc.confirmation=0",
            filter,
            "modify",
        ]);

        if !*no_alarm {
            modify_args.extend(["ALARM:"]);
        }
        if !*no_due {
            modify_args.extend(["due:"]);
        }
        if !*no_worktime {
            modify_args.extend(["WT:NonSched!"]);
        }

        let mut modify_binding = Command::new("task");
        let modify_command = modify_binding
            .args(modify_args)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit());

        if tasks_count > 2 {
            let mut modify_child = modify_command.stdin(Stdio::piped()).spawn().unwrap();

            modify_child
                .stdin
                .take()
                .unwrap()
                .write_all("all\n".as_bytes())
                .unwrap();
            modify_child.wait().unwrap();
        } else {
            modify_command.output().unwrap();
        }
    } else {
        println!("Aborting...");
    }

    Ok(())
}
pub fn und(filter: &String, unarchive: &bool) -> Result<(), FypmError> {
    let tasks = if *unarchive {
        println!("Unarchive option is true! Filtering for archived tasks...");

        get::json_by_filter(format!("(+Archived and ({}))", filter).as_str(), None)?
    } else {
        get::json_by_filter(filter, None)?
    };

    let confirmation = dialog::verify_selected_tasks(&tasks)?;

    if confirmation {
        let mut args = vec![
            "rc.verbose=0",
            "rc.confirmation=0",
            "rc.recurrence.confirmation=0",
            filter,
            "modify",
            "status:pending",
        ];

        if *unarchive {
            action::unarchive(tasks)?;

            return Ok(());
        } else {
            args.extend(["-Failed", "-Abandoned", "-NoControl"]);
        }

        let mut modify_binding = Command::new("task");
        let modify_command = modify_binding.args(args).stderr(Stdio::inherit());

        if tasks.len() > 2 {
            command::stdin_all(modify_command).unwrap();
        } else {
            modify_command.output().unwrap();
        }
    } else {
        println!("Aborting...");
    }

    Ok(())
}
pub fn recur_time(filter: &String, new_time: &String) -> Result<(), FypmError> {
    let recur_without_due_msg = "What?? A recurring task must have a due! Is it a TaskWarrior bug?";
    let date_format = "%H:%M";

    let get_task = get::json_by_filter(filter, DEFAULT_GET_JSON_OPTIONS)?;
    let received_task = get_task.get(0).unwrap();

    let old_parent_due: String;
    let parent_task_uuid: &String;
    {
        let time = NaiveTime::parse_from_str(new_time, date_format);
        match time {
            Err(_) => {
                return Err(FypmError {
                    message: format!("Specify a valid time! (Format: {})", date_format),
                    kind: FypmErrorKind::InvalidInput,
                })
            }
            Ok(_) => {}
        };
    }
    {
        if received_task.status == TaskWarriorStatus::Recurring {
            old_parent_due = parser::transform_dates_to_iso(
                received_task.due.clone().expect(recur_without_due_msg),
            )
            .unwrap();

            parent_task_uuid = &received_task.uuid;
        } else if let Some(get_parent_task_uuid) = &received_task.parent {
            let get_parent_task =
                get::json_by_filter(get_parent_task_uuid, DEFAULT_GET_JSON_OPTIONS)?;
            let parent_task = get_parent_task.get(0).unwrap();

            parent_task_uuid = get_parent_task_uuid;

            old_parent_due = parser::transform_dates_to_iso(
                parent_task.due.clone().expect(recur_without_due_msg),
            )
            .unwrap();
        } else {
            return Err(FypmError {
                message: "Selected task is not recurring!".to_string(),
                kind: FypmErrorKind::InvalidInput,
            });
        }
    }

    if let Some((date, _)) = old_parent_due.split_once("T") {
        let new_parent_due = format!("{}T{}", date, new_time);
        let pending_instances = get::json_by_filter(
            format!("+PENDING and +INSTANCE and parent:{}", parent_task_uuid).as_str(),
            None,
        );

        Command::new("task")
            .args(vec![
                "rc.confirmation=0",
                "rc.recurrence.confirmation=0",
                parent_task_uuid,
                "modify",
                format!("due:{}", new_parent_due).as_str(),
            ])
            .stderr(Stdio::inherit())
            .output()
            .unwrap();

        println!("Parent task updated! ({})", parent_task_uuid);

        for (_, task) in pending_instances.unwrap().iter().enumerate() {
            let new_instance_parsed_due = parser::transform_dates_to_iso(
                task.due.clone().expect("An instance task needs a due!"),
            )
            .unwrap();

            if let Some((date, _)) = new_instance_parsed_due.split_once("T") {
                let new_instance_due = format!("{}T{}", date, new_time);

                Command::new("task")
                    .args(vec![
                        "rc.confirmation=0",
                        "rc.recurrence.confirmation=0",
                        &task.uuid,
                        "modify",
                        format!("due:{}", new_instance_due).as_str(),
                    ])
                    .stderr(Stdio::inherit())
                    .output()
                    .unwrap();

                println!("Instance task updated! ({})", &task.uuid);
            }
        }

        Ok(())
    } else {
        Err(FypmError {
            message: "The format of the due date is invalid!".to_string(),
            kind: FypmErrorKind::InvalidInput,
        })
    }
}
