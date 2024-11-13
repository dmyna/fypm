use std::io::Error;

use chrono::NaiveTime;
use dialoguer::{console::Term, Input};
use diesel::{QueryDsl, RunQueryDsl, SqliteConnection, TextExpressionMethods};

use crate::{
    db::{models::Worktime, schema::worktimes},
    utils::verify,
};
use fypm_lib::values::err::FypmError;

pub struct WorktimeHandler;

impl WorktimeHandler {
    /// Adds a new worktime entry to the database with the given parameters.
    ///
    /// This function prompts the user for a description, style, start and end times,
    /// and polybar background and foreground colors for the worktime. It validates
    /// the time inputs to ensure they are in the correct format and that the end time
    /// is after the start time. It also validates the color inputs to ensure they are
    /// valid hexadecimal color codes. The new worktime entry is then inserted into
    /// the database.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to the SQLite connection.
    /// * `name` - A reference to a string containing the name of the worktime.
    ///
    /// # Returns
    ///
    /// * `Result<(), FypmError>` - Returns an Ok result if the worktime is successfully
    ///   added, otherwise returns an error.
    pub fn add(conn: &mut SqliteConnection, name: &String) -> Result<(), FypmError> {
        let date_format = "%H:%M";
        let term = Term::stdout();

        let description = Input::<String>::new()
            .with_prompt("Write a description for your worktime")
            .interact_text()
            .unwrap();
        term.clear_last_lines(1).unwrap();

        let style = Input::<String>::new()
            .with_prompt("What is the style of your worktime? (make sure the value exists!)")
            .interact_text()
            .unwrap();
        term.clear_last_lines(1).unwrap();

        let start_time = Input::<String>::new()
            .with_prompt("What time do you want to start this worktime? (format: HH:MM)")
            .validate_with(|input: &String| -> Result<(), &str> {
                let time = NaiveTime::parse_from_str(input.as_str(), date_format);

                match time {
                    Ok(_) => Ok(()),
                    Err(_) => Err("Not a valid time!"),
                }
            })
            .interact_text()
            .unwrap();
        term.clear_last_lines(1).unwrap();

        let end_time = Input::<String>::new()
            .with_prompt("What time do you want to end this worktime? (format: HH:MM)")
            .validate_with(|input: &String| -> Result<(), &str> {
                let time = NaiveTime::parse_from_str(input.as_str(), date_format);

                match time {
                    Ok(_) => match time.unwrap()
                        > NaiveTime::parse_from_str(start_time.as_str(), date_format).unwrap()
                    {
                        true => Ok(()),
                        false => Err("End time must be after start time!"),
                    },
                    Err(_) => Err("Not a valid time!"),
                }
            })
            .interact_text()
            .unwrap();
        term.clear_last_lines(1).unwrap();

        let polybar_background = Input::<String>::new()
            .with_prompt(
                "What color do you want to use for the background of polybar module? (HEX)",
            )
            .validate_with(|input: &String| -> Result<(), Error> {
                let hex = verify::verify_hex(input.to_string());

                match hex {
                    Ok(_) => Ok(()),
                    Err(_) => Err(hex.unwrap_err()),
                }
            })
            .interact_text()
            .unwrap();
        term.clear_last_lines(1).unwrap();

        let polybar_foreground = Input::<String>::new()
            .with_prompt(
                "What color do you want to use for the foreground of polybar module? (HEX)",
            )
            .validate_with(|input: &String| -> Result<(), &str> {
                let hex = verify::verify_hex(input.to_string());

                match hex {
                    Ok(_) => Ok(()),
                    Err(_) => Err("Not a valid hex color!"),
                }
            })
            .interact_text()
            .unwrap();
        term.clear_last_lines(1).unwrap();

        let new_worktime = Worktime {
            id: uuid::Uuid::now_v7().to_string(),
            name: name.to_string(),
            description,
            style,
            start_time,
            end_time,
            polybar_background,
            polybar_foreground,
        };

        diesel::insert_into(worktimes::table)
            .values(&new_worktime)
            .execute(conn)
            .unwrap();

        Ok(())
    }
    /// Removes a worktime from the database.
    ///
    /// # Errors
    ///
    /// This function will return an error if it fails to connect to the database or if it fails to execute
    /// the SQL query.
    pub fn remove(conn: &mut SqliteConnection, name: &String) -> Result<(), FypmError> {
        diesel::delete(worktimes::table.filter(worktimes::dsl::name.like(name)))
            .execute(conn)
            .unwrap();

        Ok(())
    }
    /// Gets a worktime from the database by name.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to the SQLite connection.
    /// * `name` - A reference to a string containing the name of the worktime.
    ///
    /// # Returns
    ///
    /// * `Result<Worktime, FypmError>` - Returns the worktime struct if the worktime exists, otherwise returns an error.
    pub fn get(conn: &mut SqliteConnection, name: &String) -> Result<Worktime, FypmError> {
        let worktime: Worktime = worktimes::dsl::worktimes
            .filter(worktimes::dsl::name.like(name))
            .first::<Worktime>(conn)
            .unwrap();

        Ok(worktime)
    }
    /// Lists all worktime entries from the database.
    ///
    /// This function retrieves all worktime records from the database and prints
    /// their names and descriptions. If no worktimes are found, it prints a message
    /// indicating that no worktimes are available.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to the SQLite connection.
    ///
    /// # Returns
    ///
    /// * `Result<(), FypmError>` - Returns an Ok result when the operation is successful,
    ///   otherwise returns an error.
    pub fn list(conn: &mut SqliteConnection) -> Result<(), FypmError> {
        let worktimes: Vec<Worktime> = worktimes::dsl::worktimes.load(conn).unwrap();

        if worktimes.is_empty() {
            println!("No worktimes found!");
        } else {
            println!("Found {} worktimes. These are:", worktimes.len());

            for worktime in worktimes {
                println!("{} - {}", worktime.name, worktime.description);
            }
        }

        Ok(())
    }
}
