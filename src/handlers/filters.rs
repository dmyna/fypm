use colored::Colorize;
use dialoguer::Input;
use diesel::{Connection, ExpressionMethods, QueryResult, RunQueryDsl, SqliteConnection};
use ratatui::style::Stylize;

use crate::{
    db::{models::Filter, schema::filters},
    utils::err::{FypmError, FypmErrorKind},
    DATABASE_URL,
};

pub struct FiltersHandler;

impl FiltersHandler {
    pub fn ensure_defaults() -> Result<(), FypmError> {
        let conn = &mut SqliteConnection::establish(DATABASE_URL.as_str()).unwrap();

        let filters = FiltersHandler::get_filters(conn).unwrap();

        let defaults = vec![Filter {
            id: uuid::Uuid::now_v7().to_string(),
            name: "late_alarm".to_string(),
            filter: "(ALARM.before:now -PARENT -COMPLETED -DELETED)".to_string(),
        }];

        for default in defaults {
            let mut filter_exists = false;

            for filter in &filters {
                if default.name == filter.name {
                    filter_exists = true;
                }
            }

            if !filter_exists {
                diesel::insert_into(filters::table)
                    .values(default)
                    .execute(conn)
                    .unwrap();
            }
        }

        Ok(())
    }

    pub fn verify_filter_already_exists(
        input: &String,
        filters: &Vec<Filter>,
    ) -> Result<(), String> {
        for filter in filters {
            if filter.name == *input {
                return Err(format!(
                    "Filter with name '{}' already exists! Choose a different name.",
                    input.clone()
                ));
            }
        }

        Ok(())
    }
    pub fn verify_filter_not_exists(input: &String, filters: &Vec<Filter>) -> Result<(), String> {
        for filter in filters {
            if filter.name == *input {
                return Ok(());
            }
        }

        Err(format!(
            "Filter with name '{}' does not exist! Choose a different name.",
            input.clone()
        ))
    }

    pub fn get_filters(conn: &mut SqliteConnection) -> QueryResult<Vec<Filter>> {
        filters::dsl::filters.load(conn)
    }

    pub fn add(conn: &mut SqliteConnection) -> Result<(), FypmError> {
        let filters = FiltersHandler::get_filters(conn).unwrap();

        let name = Input::<String>::new()
            .with_prompt("Write a name for your filter")
            .validate_with(|input: &String| -> Result<(), String> {
                FiltersHandler::verify_filter_already_exists(input, &filters)
            })
            .interact_text()
            .unwrap()
            .trim()
            .to_string();

        let new_filter = Input::<String>::new()
            .with_prompt("Write your filter")
            .interact_text()
            .unwrap()
            .trim()
            .to_string();

        diesel::insert_into(filters::table)
            .values(Filter {
                id: uuid::Uuid::now_v7().to_string(),
                name: name.clone(),
                filter: new_filter,
            })
            .execute(conn)
            .unwrap();

        Ok(())
    }
    pub fn remove(conn: &mut SqliteConnection) -> Result<(), FypmError> {
        let filters = FiltersHandler::get_filters(conn).unwrap();

        let name = Input::<String>::new()
            .with_prompt("Write the name of the filter you want to remove")
            .validate_with(|input: &String| -> Result<(), String> {
                FiltersHandler::verify_filter_not_exists(input, &filters)
            })
            .interact_text()
            .unwrap()
            .trim()
            .to_string();

        diesel::delete(filters::table)
            .filter(filters::name.eq(name))
            .execute(conn)
            .unwrap();

        Ok(())
    }

    pub fn edit(conn: &mut SqliteConnection) -> Result<(), FypmError> {
        let filters = FiltersHandler::get_filters(conn).unwrap();

        let name = Input::<String>::new()
            .with_prompt("Write the name of the filter you want to edit")
            .validate_with(|input: &String| -> Result<(), String> {
                FiltersHandler::verify_filter_not_exists(input, &filters)
            })
            .interact_text()
            .unwrap()
            .trim()
            .to_string();

        let new_name = Input::<String>::new()
            .with_prompt("Write the new name (enter a space to keep the same)")
            .validate_with(|input: &String| -> Result<(), String> {
                if input.trim().is_empty() {
                    return Ok(());
                }

                FiltersHandler::verify_filter_already_exists(input, &filters)
            })
            .interact_text()
            .unwrap()
            .trim()
            .to_string();

        let new_filter = Input::<String>::new()
            .with_prompt("Write the new filter (enter a space to keep the same)")
            .validate_with(|input: &String| -> Result<(), String> {
                if input.trim().is_empty() {
                    return Ok(());
                }

                FiltersHandler::verify_filter_already_exists(input, &filters)
            })
            .interact_text()
            .unwrap()
            .trim()
            .to_string();

        let mut set_values: (
            diesel::dsl::Eq<filters::columns::name, String>,
            diesel::dsl::Eq<filters::columns::filter, String>,
        ) = (
            filters::name.eq(new_name.clone()),
            filters::filter.eq(new_filter.clone()),
        );

        if !new_name.is_empty() && new_filter.is_empty() {
            for filter in filters {
                if filter.name == name {
                    set_values = (
                        filters::name.eq(new_name),
                        filters::filter.eq(filter.filter),
                    );
                    break;
                }
            }
        } else if new_name.is_empty() && !new_filter.is_empty() {
            set_values = (
                filters::name.eq(name.clone()),
                filters::filter.eq(new_filter),
            );
        } else {
            return Err(FypmError {
                message: "You must write something to edit!".to_string(),
                kind: FypmErrorKind::InvalidInput,
            });
        }

        diesel::update(filters::table)
            .filter(filters::name.eq(name))
            .set(set_values)
            .execute(conn)
            .unwrap();

        Ok(())
    }
    pub fn list(conn: &mut SqliteConnection) -> Result<(), FypmError> {
        let filters: Vec<Filter> = FiltersHandler::get_filters(conn).unwrap();

        println!("{}", "These are the current filters:".bright_white().bold());
        for filter in filters {
            println!(
                "-   {} -> {}",
                filter.name.bright_white(),
                filter.filter.gray()
            );
        }

        Ok(())
    }
}
