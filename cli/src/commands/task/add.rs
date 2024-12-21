////////////////////////////////////////////////////////////////////////////////
// fypm - The Dark Souls of productivity.
// Copyright (C) 2023-2024 Rikagaku <contact.rikagaku@gmail.com>
// Copyright (C) 2023-2024 Myna <contact@devmyna.xyz>
//
// fypm is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// fypm is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with fypm. If not, see <https://www.gnu.org/licenses/>.
//
////////////////////////////////////////////////////////////////////////////////

use std::process::Command;
use std::str;

use chrono::{DateTime, Datelike, Local};
use dialoguer::Confirm;

use crate::utils::get;
use crate::commands::TaSequenceTypes;

use fypm_lib::values::{
    constants::DEFAULT_GET_JSON_OPTIONS,
    err::{FypmError, FypmErrorKind},
    structs::TaskWarriorExported,
};

/// Creates a new task and return its uuid.
///
/// # Arguments
///
/// * `description`: The description of the task.
/// * `project`: The project of the task.
/// * `style`: The style of the task.
/// * `r#type`: The type of the task.
/// * `other_args`: Any other argument to be passed to the `task` command.
/// * `skip_confirmation`: If `true`, the function will not prompt the user for confirmation.
///
/// # Examples
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
/// Adds a subtask to a specified mother task.
///
/// If the mother task is part of a sequence, the subtask will be added to the sequence
/// and linked with the previous sequence subtask. If not, the subtask is simply linked
/// to the mother task.
///
/// # Arguments
///
/// * `mother_task`: The UUID of the mother task to which the subtask will be added.
/// * `other_args`: A vector containing the arguments for the subtask, including its
///   description, style, type, and any additional parameters.
/// * `skip_confirmation`: If `true`, skips the confirmation prompt before creating
///   the subtask.
///
/// # Returns
///
/// * `Result<String, FypmError>` - The UUID of the created subtask if successful,
///   or a `FypmError` if an error occurs.
///
/// # Errors
///
/// * `FypmErrorKind::InvalidInput` - If an incorrect number of arguments is provided.
/// * `FypmErrorKind::WrongInitialization` - If the mother task is part of a sequence
///   but lacks a sequence ID.
/// * `FypmErrorKind::NoTasksFound` - If no previous sequence subtask is found.
/// * Other error kinds may be returned depending on the underlying operations.
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
    /// Create a sequence of tasks.
    ///
    /// The sequence type is defined by the field `seq_type`.
    /// The sequence will have as many tasks as the difference between `initial_number` and `last_number`.
    /// The sequence will have the tag `ST_<tag>` and `seq_type.to_string()`.
    /// The sequence will be linked to the mother task.
    /// The mother task will have the tag `Sequence`.
    /// If `season` is `Some`, the mother task's description will be `description (Season <season>)`.
    /// If `last_season_id` is `Some`, the first task of the sequence will have the tag `SEQ_PREVIOUS:<last_season_id>`.
    /// The next task of each task will have the tag `SEQ_NEXT:<next_task_uuid>`.
    /// The current task of the mother task will have the tag `SEQ_CURRENT:<first_task_uuid>`.
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
/// Creates a new birthday task or event in the system.
///
/// This function calculates the next occurrence of a given birthday based on the current date.
/// It constructs a task with specific attributes such as work time, recurrence, goal, and due date,
/// then returns the UUID of the created task.
///
/// # Arguments
///
/// * `birthday_person` - A reference to a string representing the name of the person whose birthday
///   event is being created.
/// * `date` - A reference to a string representing the date of the birthday in "MM-DD" format.
///
/// # Returns
///
/// A `Result` containing a UUID string of the newly created birthday task or an `FypmError` if
/// the task creation fails.
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
/// Creates a new playlist task with specified subtasks.
///
/// This function initializes a new playlist task using a specified style and project.
/// It adds subtasks for a cover, description, and a number of songs to the playlist.
///
/// # Arguments
///
/// * `playlist_name` - A reference to a string representing the name of the playlist.
/// * `length` - A reference to an unsigned 16-bit integer indicating the number of songs in the playlist.
///
/// # Returns
///
/// A `Result` containing a UUID string of the newly created playlist task or an `FypmError` if
/// the task creation fails.
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
