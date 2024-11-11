use std::io::Error;

use chrono::NaiveTime;
use dialoguer::{console::Term, Input};
use diesel::{QueryDsl, RunQueryDsl, SqliteConnection, TextExpressionMethods};

use crate::{
    db::{models::Worktime, schema::worktimes},
    utils::verify,
    values::err::FypmError,
};

pub struct WorktimeHandler;

impl WorktimeHandler {
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
    pub fn remove(conn: &mut SqliteConnection, name: &String) -> Result<(), FypmError> {
        diesel::delete(worktimes::table.filter(worktimes::dsl::name.like(name)))
            .execute(conn)
            .unwrap();

        Ok(())
    }
    pub fn get(conn: &mut SqliteConnection, name: &String) -> Result<Worktime, FypmError> {
        let worktime: Worktime = worktimes::dsl::worktimes
            .filter(worktimes::dsl::name.like(name))
            .first::<Worktime>(conn)
            .unwrap();

        Ok(worktime)
    }
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
