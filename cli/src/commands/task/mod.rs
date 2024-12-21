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

use std::process::{Command, Stdio};

use dialoguer::Confirm;

use fypm_lib::values::{
    err::{FypmError, FypmErrorKind},
    structs::TaskWarriorExported,
};

use crate::{
    func::action,
    utils::get,
    commands::{TaProjectActions, TaAbandonTags},
};

pub mod add;
pub mod list;
pub mod update;

/// Handles actions related to projects.
///
/// # Actions
///
/// * `list`: Lists all projects. If a project name is provided, it will filter the output.
/// * `add`: Adds a new project. If a project name is not provided, it will return an error.
/// * `archive`: Archives all completed tasks of a given project. If a project name is not provided, it will return an error.
/// * `unarchive`: Unarchives all archived tasks of a given project. If a project name is not provided, it will return an error.
pub fn task_project(action: &TaProjectActions, arg: &Option<String>) -> Result<(), FypmError> {
    let no_project_specified = FypmError {
        message: "Please provide a project name!".to_string(),
        kind: FypmErrorKind::InvalidInput,
    };

    match *action {
        TaProjectActions::List => {
            let mut args = Vec::new();

            if let Some(filter) = arg {
                args.extend([format!("project:{}", filter)]);
            }

            args.extend(["projects".to_string()]);

            Command::new("task")
                .args(args)
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .output()
                .unwrap();
        }
        TaProjectActions::Add => {
            if let Some(project) = arg {
                let confirmation: bool = Confirm::new()
                    .with_prompt(format!("Do you want to add '{}' project?", project))
                    .interact()
                    .unwrap();

                if confirmation {
                    add::new(
                        &"Project Marker".to_string(),
                        project,
                        &" ".to_string(),
                        &"Continuous".to_string(),
                        &Some(vec!["+FYPM".to_string()]),
                        &true,
                    )?;
                }
            } else {
                return Err(no_project_specified);
            }
        }
        TaProjectActions::Archive => {
            if let Some(project) = arg {
                let confirmation: bool = Confirm::new()
                    .with_prompt(format!("Do you want to archive '{}' project?", project))
                    .interact()
                    .unwrap();

                if confirmation {
                    update::abandon(
                        &TaAbandonTags::Archived,
                        &format!("(project:{} and -DELETED and -COMPLETED)", project),
                        &None,
                        &None,
                    )?;
                }
            } else {
                return Err(no_project_specified);
            }
        }
        TaProjectActions::Unarchive => {
            if let Some(project) = arg {
                let confirmation: bool = Confirm::new()
                    .with_prompt(format!("Do you want to unarchive '{}' project?", project))
                    .interact()
                    .unwrap();

                if confirmation {
                    println!("Unarchive option is true! Filtering for archived tasks...");

                    let tasks: Vec<TaskWarriorExported> = get::json_by_filter(
                        format!("(project:{} and +Archived)", project).as_str(),
                        None,
                    )?;

                    action::unarchive(tasks)?;
                }
            } else {
                return Err(no_project_specified);
            }
        }
    }

    Ok(())
}
