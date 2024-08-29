use std::process::Command;
use std::str;

use chrono::{DateTime, Datelike, Local};
use dialoguer::Confirm;

use crate::{
    utils::get,
    values::{
        constants::DEFAULT_GET_JSON_OPTIONS,
        enums::TaSequenceTypes,
        err::{FypmError, FypmErrorKind},
        structs::TaskWarriorExported,
    },
};

pub fn new(
    description: &String,
    project: &String,
    style: &String,
    r#type: &String,
    other_args: &Option<Vec<String>>,
    skip_confirmation: &bool,
) -> Result<String, FypmError> {
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
            return Err(FypmError {
                message: "Aborted".to_string(),
                kind: FypmErrorKind::Aborted,
            });
        }
    }

    let mut args = vec![
        "rc.verbose=new-id".to_string(),
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

    let id: String;
    if let Ok(output) = execute {
        if output.status.success() {
            let stdout = str::from_utf8(&output.stdout).unwrap();

            id = stdout
                .trim()
                .replace("Created task ", "")
                .replace(".", "")
                .to_string();
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

    let uuid = get::get_uuids_by_filter(&id, DEFAULT_GET_JSON_OPTIONS)?[0].clone();

    println!("Created task with id \"{}\"! ({})", id, uuid);

    Ok(uuid)
}
pub fn subtask(
    mother_task: &String,
    other_args: &Vec<String>,
    skip_confirmation: &bool,
) -> Result<String, FypmError> {
    fn create_subtask(
        mother_task_json: &TaskWarriorExported,
        other_args: &Vec<String>,
        skip_confirmation: &bool,
    ) -> Result<String, FypmError> {
        let project: &String;
        let description = other_args.get(0).unwrap();
        let style = other_args.get(1).unwrap();
        let r#type = other_args.get(2).unwrap();

        if let Some(project_arg) = &mother_task_json.project {
            project = project_arg;
        } else {
            panic!("The specified mother doesn't have a project setted... Are you writing this stuff right?");
        }

        let args = other_args.get(3..).unwrap().to_vec();

        let uuid = new(
            description,
            project,
            style,
            r#type,
            &Some(args),
            skip_confirmation,
        )?;

        Ok(uuid)
    }

    let wrong_arguments_error = FypmError {
        message: "You specified a wrong number of arguments! You don't know how to read documentation, do you? :P".to_string(),
        kind: FypmErrorKind::InvalidInput
    };

    let subtask: String;

    let get_mother_task_json = get::json_by_filter(mother_task, DEFAULT_GET_JSON_OPTIONS)?;
    let mother_task_json = get_mother_task_json.get(0).unwrap();

    if mother_task_json.tags.is_some()
        && mother_task_json
            .tags
            .as_ref()
            .unwrap()
            .contains(&"Sequence".to_string())
    {
        let tags = mother_task_json.tags.as_ref().unwrap();
        let mut seq_id = "unknown".to_string();
        {
            for tag in tags {
                if tag.starts_with("ST_") {
                    seq_id = tag.to_string();
                }
            }

            if seq_id == "unknown" {
                return Err(FypmError {
                    message: "You are trying to add a subtask to a sequence, but this sequence doesn't have a Sequence ID".to_string(),
                    kind: FypmErrorKind::WrongInitialization,
                });
            }
        }

        let get_last_seq_subtask = get::json_by_filter(
            format!(
                "+Sequence and +{} and SEQ_PREVIOUS.any: and SEQ_NEXT.none: and MOTHER:{}",
                seq_id, mother_task_json.uuid
            )
            .as_str(),
            DEFAULT_GET_JSON_OPTIONS,
        );

        if other_args.len() == 1 {
            let get_subtask_uuid =
                get::get_uuids_by_filter(other_args.get(0).unwrap(), DEFAULT_GET_JSON_OPTIONS)?;
            let subtask_uuid = get_subtask_uuid.get(0).unwrap();

            subtask = subtask_uuid.to_string();
        } else if other_args.len() >= 3 {
            subtask = create_subtask(mother_task_json, other_args, skip_confirmation)?;
        } else {
            return Err(wrong_arguments_error);
        }

        let mut new_seq_subtask_args = vec![
            subtask.clone(),
            "modify".to_string(),
            "+SUBTASK".to_string(),
            "+Sequence".to_string(),
            format!("+{}", seq_id),
        ];

        match get_last_seq_subtask {
            Ok(last_seq_subtask_vec) => {
                let last_seq_subtask = last_seq_subtask_vec.get(0).unwrap();

                Command::new("task")
                    .args([
                        &last_seq_subtask.uuid,
                        &"modify".to_string(),
                        &format!("SEQ_NEXT:{}", subtask),
                    ])
                    .output()
                    .unwrap();

                new_seq_subtask_args.push(format!("SEQ_PREVIOUS:{}", last_seq_subtask.uuid));
            }
            Err(err) => match err.kind {
                FypmErrorKind::NoTasksFound => {}
                _ => return Err(err),
            },
        }

        Command::new("task")
            .args(new_seq_subtask_args)
            .output()
            .unwrap();
    } else {
        if other_args.len() == 1 {
            let get_subtask_uuid =
                get::get_uuids_by_filter(other_args.get(0).unwrap(), DEFAULT_GET_JSON_OPTIONS)?;
            let subtask_uuid = get_subtask_uuid.get(0).unwrap();

            subtask = subtask_uuid.to_string();
        } else if other_args.len() >= 3 {
            subtask = create_subtask(mother_task_json, other_args, skip_confirmation)?;
        } else {
            return Err(wrong_arguments_error);
        }
    }

    // Define mother task as a mother task and set the subtask to mother task
    {
        Command::new("task")
            .args([mother_task.as_str(), "modify", "STATE:Info", "+MOTHER"])
            .output()
            .unwrap();
        println!("Mother task setted.");

        Command::new("task")
            .args([
                &subtask,
                &"modify".to_string(),
                &format!("MOTHER:{}", mother_task_json.uuid),
                &"+SUBTASK".to_string(),
            ])
            .output()
            .unwrap();
    }
    println!(
        "Subtask added to its MOTHER '{}'!",
        mother_task_json.description
    );

    Ok(subtask)
}
pub fn sequence(
    seq_type: &TaSequenceTypes,
    style: &String,
    description: &String,
    project: &String,
    tag: &String,
    initial_number: &usize,
    last_number: &usize,
    season: &Option<String>,
    last_season_id: &Option<String>,
) -> Result<(), FypmError> {
    let mother_task_uuid: String;
    let mother_description: String;
    let final_tag = format!("+ST_{}", tag);
    let final_tag_type = format!("+{}", seq_type.to_string());

    if let Some(season) = season {
        mother_description = format!("{} (Season {})", description, season)
    } else {
        mother_description = format!("{}", description);
    }

    {
        let uuid = new(
            &mother_description,
            &project.to_string(),
            &style,
            &"Objective".to_string(),
            &Some(vec![
                "+Sequence".to_string(),
                final_tag.clone(),
                final_tag_type.clone(),
            ]),
            &true,
        )?;

        mother_task_uuid = uuid;
    }

    let mut previous_task_uuid: String = "".to_string();

    for i in *initial_number..=*last_number {
        let mother_task_uuid = &mother_task_uuid;
        let subtask_description: String;

        match seq_type {
            TaSequenceTypes::Book => {
                subtask_description = format!("Chapter {}", i);
            }
            _ => {
                if let Some(season) = season {
                    subtask_description = format!("S{}E{}", season, i);
                } else {
                    subtask_description = format!("E{}", i);
                }
            }
        }

        let mut args = vec![
            subtask_description.clone(),
            style.clone(),
            "Objective".to_string(),
            final_tag.clone(),
            final_tag_type.clone(),
            "+Sequence".to_string(),
        ];

        if i == *initial_number {
            if let Some(last_season_id) = last_season_id {
                let get_last_season_json =
                    get::json_by_filter(&last_season_id, DEFAULT_GET_JSON_OPTIONS).unwrap();
                let last_season_json = get_last_season_json.get(0).unwrap();

                args.push(format!("SEQ_PREVIOUS:{}", last_season_json.uuid));
            }

            let current_task_uuid = subtask(&mother_task_uuid, &args, &true).unwrap();

            if let Some(last_season_id) = last_season_id {
                Command::new("task")
                    .args([
                        last_season_id,
                        &"modify".to_string(),
                        &format!("SEQ_PREVIOUS:{}", current_task_uuid),
                    ])
                    .output()
                    .unwrap();
            }

            Command::new("task")
                .args([
                    mother_task_uuid,
                    &"modify".to_string(),
                    &format!("SEQ_CURRENT:{}", current_task_uuid),
                ])
                .output()
                .unwrap();

            previous_task_uuid = current_task_uuid;
        } else {
            if previous_task_uuid == "".to_string() {
                panic!("previous_task_uuid is empty!");
            }

            let current_task_uuid = subtask(&mother_task_uuid, &args, &true).unwrap();

            Command::new("task")
                .args([
                    &current_task_uuid,
                    &"modify".to_string(),
                    &format!("SEQ_PREVIOUS:{}", previous_task_uuid),
                ])
                .output()
                .unwrap();
            Command::new("task")
                .args([
                    previous_task_uuid,
                    "modify".to_string(),
                    format!("SEQ_NEXT:{}", &current_task_uuid),
                ])
                .output()
                .unwrap();

            previous_task_uuid = current_task_uuid;
        }
    }

    Ok(())
}
pub fn birthday(birthday_person: &String, date: &String) -> Result<String, FypmError> {
    let current_year = Local::now().year().to_string();

    let date =
        DateTime::parse_from_rfc3339(format!("{}-{}T23:59:59Z", current_year, date).as_str())
            .unwrap()
            .date_naive();

    let current_date = Local::now().date_naive();

    let mut final_date: String = "".to_string();

    if current_date <= date {
        final_date = date.to_string();
    } else {
        let add_a_year = date.with_year(date.year() + 1);

        if let Some(new_date) = add_a_year {
            final_date = new_date.to_string();
        }
    }

    let uuid = new(
        &format!("{}'s Birthday", birthday_person),
        &"Social.Events".to_string(),
        &"Dionysian".to_string(),
        &"Event".to_string(),
        &Some(vec![
            "WT:AllDay!".to_string(),
            "recur:yearly".to_string(),
            format!("GOAL:{}T00:00:00", &final_date),
            format!("due:{}T23:59:59", &final_date),
        ]),
        &true,
    )?;

    Ok(uuid)
    //Ok(uuid)
}
pub fn playlist(playlist_name: &String, length: &u16) -> Result<String, FypmError> {
    let style = "Dionysian".to_string();

    let mother_uuid = new(
        &playlist_name,
        &"Music.Playlist".to_string(),
        &style,
        &"Objective".to_string(),
        &None,
        &true,
    )?;

    subtask(
        &mother_uuid,
        &vec!["Cover".to_string(), style.clone()],
        &true,
    )?;
    subtask(
        &mother_uuid,
        &vec!["Description".to_string(), style.clone()],
        &true,
    )?;
    subtask(
        &mother_uuid,
        &vec![format!("Songs ({})", length), style],
        &true,
    )?;

    Ok(mother_uuid)
}
