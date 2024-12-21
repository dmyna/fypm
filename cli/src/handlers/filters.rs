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

use colored::Colorize;
use dialoguer::Input;
use diesel::{Connection, ExpressionMethods, QueryResult, RunQueryDsl, SqliteConnection};
use ratatui::style::Stylize;

use crate::{
    db::{models::Filter, schema::filters},
    DATABASE_URL,
};
use fypm_lib::values::err::{FypmError, FypmErrorKind};

pub struct FiltersHandler;

impl FiltersHandler {
    /// Ensures that the default filters exist in the database.
    ///
    /// The function checks if the default filters exist in the database. If a default filter does not exist,
    /// it creates it. The default filters are:
    ///
    /// - `late_alarm`: `(ALARM.before:now -PARENT -COMPLETED -DELETED)`.
    ///
    /// # Errors
    ///
    /// This function will return an error if it fails to connect to the database or if it fails to execute
    /// the SQL query.
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

    /// Checks if a filter with the given name already exists in the list of filters.
    ///
    /// This function iterates over the provided list of `filters` and checks if any filter
    /// has a name that matches the `input` name. If a matching filter is found, it returns
    /// an error indicating that the filter already exists.
    ///
    /// # Arguments
    ///
    /// * `input` - A reference to a `String` representing the name of the filter to check.
    /// * `filters` - A reference to a `Vec<Filter>` representing the list of filters to search.
    ///
    /// # Returns
    ///
    /// * `Result<(), String>` - Returns `Ok(())` if no matching filter is found, otherwise returns
    /// an `Err` with a message indicating that a filter with the given name already exists.
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
    /// Checks if a filter with the given name does not exist in the list of filters.
    ///
    /// This function iterates over the provided list of `filters` and checks if any filter
    /// has a name that matches the `input` name. If a matching filter is found, it returns
    /// an `Ok(())`, otherwise returns an `Err` with a message indicating that a filter with
    /// the given name does not exist.
    ///
    /// # Arguments
    ///
    /// * `input` - A reference to a `String` representing the name of the filter to check.
    /// * `filters` - A reference to a `Vec<Filter>` representing the list of filters to search.
    ///
    /// # Returns
    ///
    /// * `Result<(), String>` - Returns `Ok(())` if a matching filter is found, otherwise returns
    /// an `Err` with a message indicating that a filter with the given name does not exist.
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

    /// Retrieves all filters from the database.
    ///
    /// # Errors
    ///
    /// This function will return an error if it fails to connect to the database or if it fails to execute
    /// the SQL query.
    pub fn get_filters(conn: &mut SqliteConnection) -> QueryResult<Vec<Filter>> {
        filters::dsl::filters.load(conn)
    }

    /// Adds a filter to the database.
    ///
    /// This function will ask for a name and a filter and will verify that the name does not already exist in the database.
    /// If the name does not exist, it will add the filter to the database.
    ///
    /// # Errors
    ///
    /// This function will return an error if it fails to connect to the database or if it fails to execute
    /// the SQL query.
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
    /// Removes a filter from the database.
    ///
    /// This function will ask for a name and will verify that the name exists in the database.
    /// If the name exists, it will remove the filter from the database.
    ///
    /// # Errors
    ///
    /// This function will return an error if it fails to connect to the database or if it fails to execute
    /// the SQL query.
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

    /// Edits a filter in the database.
    ///
    /// This function will ask for a name and will verify that the name exists in the database.
    /// If the name exists, it will ask for a new name and a new filter.
    /// If the new name is empty, it will keep the same name.
    /// If the new filter is empty, it will keep the same filter.
    /// If the new name is not empty and the new filter is not empty, it will update the filter in the database.
    ///
    /// # Errors
    ///
    /// This function will return an error if it fails to connect to the database or if it fails to execute
    /// the SQL query.
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
    /// Lists all filters in the database.
    ///
    /// This function retrieves all filters from the database and prints them to the console.
    /// Each filter is displayed with its name and filter details.
    ///
    /// # Errors
    ///
    /// This function will return an error if it fails to connect to the database or if it fails to execute
    /// the SQL query.
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
