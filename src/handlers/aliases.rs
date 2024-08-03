use dialoguer::{Confirm, Input};

use crate::utils::constants::DEFAULT_GET_JSON_OPTIONS;
use crate::utils::err::{FypmError, FypmErrorKind};
use crate::utils::get;
use crate::utils::structs::TaskWarriorExported;

pub struct AliasesHandler;

impl AliasesHandler {
    pub fn get_tasks_by_alias_existence() -> Result<[Vec<TaskWarriorExported>; 2], FypmError> {
        let mut tasks_with_alias: Vec<TaskWarriorExported> = Vec::new();
        let mut tasks_without_alias: Vec<TaskWarriorExported> = Vec::new();

        let tasks = get::get_json_by_filter("(TYPE:Continuous and status:pending)", None)?;

        for task in tasks {
            if let Some(_) = &task.alias {
                tasks_with_alias.push(task);
            } else {
                tasks_without_alias.push(task);
            }
        }

        Ok([tasks_with_alias, tasks_without_alias])
    }
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

    pub fn add(filter: &String) -> Result<(), FypmError> {
        let get_task = get::get_json_by_filter(filter, DEFAULT_GET_JSON_OPTIONS)?;
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

                let tasks_with_this_alias = get::get_json_by_filter(
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
