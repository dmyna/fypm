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

use dialoguer::{Confirm, Input};

use fypm_lib::values::constants::DEFAULT_GET_JSON_OPTIONS;
use fypm_lib::values::err::{FypmError, FypmErrorKind};
use crate::utils::get;
use fypm_lib::values::structs::TaskWarriorExported;

pub struct AliasesHandler;

impl AliasesHandler {
    /// Separates continuous tasks into two vectors, one with aliases and one without.
    ///
    /// # Returns
    /// A vector containing two elements: a vector of tasks with aliases and a vector of tasks without aliases.
    pub fn get_tasks_by_alias_existence() -> Result<[Vec<TaskWarriorExported>; 2], FypmError> {
        let mut tasks_with_alias: Vec<TaskWarriorExported> = Vec::new();
        let mut tasks_without_alias: Vec<TaskWarriorExported> = Vec::new();

        let tasks = get::json_by_filter("(TYPE:Continuous and status:pending)", None)?;

        for task in tasks {
            if let Some(_) = &task.alias {
                tasks_with_alias.push(task);
            } else {
                tasks_without_alias.push(task);
            }
        }

        Ok([tasks_with_alias, tasks_without_alias])
    }
    /// Verifies if all tasks with Continuous TYPE have an alias set.
    ///
    /// If there are tasks without aliases, it will return an error.
    /// This function is used internally to ensure that all Continuous tasks have an alias.
    ///
    /// # Returns
    /// * `Result<(), FypmError>` - Returns `Ok(())` if all Continuous tasks have an alias, or a `FypmError` if some tasks are missing an alias.
    ///
    /// # Errors
    /// If there are tasks with Continuous TYPE without an alias, it will return a `FypmError` with `kind` set to `TaskTypeError`.
    pub fn ensure_aliases_tasks() -> Result<(), FypmError> {
        let [_, tasks_without_alias] = Self::get_tasks_by_alias_existence()?;

        if tasks_without_alias.len() > 0 {
            Err(FypmError {
                message: "Tasks with Continuous TYPE must have an alias! Run `fypm alias --help to verify them`".to_string(),
                kind: FypmErrorKind::TaskTypeError,
            })
        } else {
            Ok(())
        }
    }
    /// Adds an alias to a task.
    ///
    /// It will ask to input an alias for the task and will verify if the alias is already used by another Continuous task.
    /// If the alias is already used, it will show a message and ask if you want to try again.
    /// If the alias is not used, it will add the alias to the task.
    ///
    /// # Returns
    /// * `Result<(), FypmError>` - Returns `Ok(())` if the alias was added successfully, or a `FypmError` if something went wrong.
    pub fn add(filter: &String) -> Result<(), FypmError> {
        let get_task = get::json_by_filter(filter, DEFAULT_GET_JSON_OPTIONS)?;
        let task = get_task
            .get(0)
            .unwrap();

        if let Some(alias) = &task.alias {
            println!("Task {} already has alias {}. Are you trying to change? You can use `fypm alias change`!", task.uuid, alias);
        } else {
            let valid_alias = false;

            while !valid_alias {
                let get_input = Input::<String>::new()
                    .with_prompt(format!("Write an alias for `{}` task:", task.description))
                    .validate_with(|input: &String| -> Result<(), &str> {
                        if input.is_empty() {
                            Err("Alias cannot be empty!")
                        } else if input.len() > 5 {
                            Err("Alias cannot be longer than 5 characters!")
                        } else {
                            Ok(())
                        }
                    })
                    .interact_text()
                    .unwrap();

                let tasks_with_this_alias = get::json_by_filter(
                    &format!(
                        "(TYPE:Continuous and status:pending and ALIAS:\"{}\")",
                        get_input
                    ),
                    DEFAULT_GET_JSON_OPTIONS,
                )
                .unwrap();

                if tasks_with_this_alias.len() > 0 {
                    eprintln!("Oh no! You have a task with this alias!");

                    let try_again = Confirm::new()
                        .with_prompt("Do you want to try again?")
                        .interact()
                        .unwrap();

                    if !try_again {
                        return Ok(());
                    }
                } else {
                    unimplemented!()
                }
            }
        }

        Ok(())
    }

    //pub fn change(self, name: &String) -> Result<(), FypmError> {}
}

/// Verifies and reports the status of Continuous tasks with respect to their alias assignment.
///
/// This function retrieves Continuous tasks and categorizes them into those with aliases and 
/// those without. It prints the count of tasks with aliases and provides a warning if there 
/// are tasks missing aliases. If any tasks are missing aliases, the function prompts the user 
/// to list them, displaying their UUIDs and descriptions if the user confirms.
///
/// # Returns
/// * `Result<(), FypmError>` - Returns `Ok(())` if the operation is successful or a `FypmError` 
///   if an error occurs during task retrieval.
pub fn verify_aliases_tasks() -> Result<(), FypmError> {
    let [tasks_with_alias, tasks_without_alias] = AliasesHandler::get_tasks_by_alias_existence()?;

    println!("{} tasks with alias", tasks_with_alias.len());

    if tasks_without_alias.len() > 0 {
        eprintln!(
            "Oh no! You have {} tasks without alias! Fix it soon.",
            tasks_without_alias.len()
        );

        let confirmation = Confirm::new()
            .with_prompt("Do you want to list them?")
            .interact()
            .unwrap();

        if confirmation {
            for task in tasks_without_alias {
                println!("{} - {}", task.uuid, task.description);
            }
        }
    } else {
        println!("You do not have any tasks without alias, congrats! 🎉");
    }

    Ok(())
}
