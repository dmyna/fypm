use colored::Colorize;
use dialoguer::Input;
use diesel::{Connection, QueryResult, RunQueryDsl, SqliteConnection};
use ratatui::style::Stylize;

use crate::{
    db::{models::Filter, schema::filters},
    utils::err::FypmError,
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

    pub fn get_filters(conn: &mut SqliteConnection) -> QueryResult<Vec<Filter>> {
        filters::dsl::filters.load(conn)
    }

    pub fn add(conn: &mut SqliteConnection) -> Result<(), FypmError> {
        let filters = FiltersHandler::get_filters(conn).unwrap();

        let name = Input::<String>::new()
            .with_prompt("Write a name for your filter: ")
            .validate_with(|input: &String| -> Result<(), String> {
                for filter in &filters {
                    if filter.name == *input {
                        return Err(format!(
                            "Filter with name '{}' already exists! Choose a different name.",
                            input.clone()
                        ));
                    }
                }

                Ok(())
            })
            .interact_text()
            .unwrap()
            .trim()
            .to_string();

        let new_filter = Input::<String>::new()
            .with_prompt("Write your filter: ")
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
