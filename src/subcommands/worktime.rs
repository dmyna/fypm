use chrono::NaiveTime;
use dialoguer::{console::Term, Input};

use crate::handlers::data_bowl;
use crate::utils::verify;

#[derive(Debug)]
struct Worktime {
    start_time: String,
    end_time: String,
    polybar_background: String,
    polybar_foreground: String,
}

pub struct WorktimeHandler;
impl WorktimeHandler {
    pub fn ensure_worktime_data_bowl() {
        let data_bowl_name = String::from("worktime");

        let verify_existence = data_bowl::DataBowlHandler::verify_by_name(&data_bowl_name).unwrap();

        if verify_existence == false {
            data_bowl::DataBowlHandler::create(
                &data_bowl_name,
                &String::from("The databowl of your worktimes!"),
            )
            .unwrap();
        }
    }
    pub fn add(matches: &clap::ArgMatches) {
        let date_format = "%H:%M";
        let args = matches.get_many::<String>("ACTIONARGS").unwrap();

        if args.len() > 1 {
            panic!("Too much arguments!");
        } else if args.len() < 1 {
            panic!("Not enough arguments!");
        }

        let name = args.clone().nth(0).unwrap().to_string();

        let verify_existence =
            data_bowl::DataBowlHandler::verify_by_name(&name.to_string()).unwrap();
        if verify_existence == true {
            panic!("Worktime {} already exists!", name);
        } else {
            let term = Term::stdout();

            let description = Input::<String>::new()
                .with_prompt("Write a description for your worktime")
                .interact_text()
                .unwrap();
            term.clear_last_lines(1).unwrap();

            let start_time = Input::<String>::new()
                .with_prompt("What time do you want to start this worktime?")
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
                .with_prompt("What time do you want to end this worktime?")
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
                .with_prompt("What color do you want to use for the background of polybar module?")
                .validate_with(|input: &String| -> Result<(), String> {
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
                .with_prompt("What color do you want to use for the foreground of polybar module?")
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
                start_time,
                end_time,
                polybar_background,
                polybar_foreground,
            };
        }
    }
}

pub fn match_action(matches: &clap::ArgMatches) -> Result<(), String> {
    match matches.get_one::<String>("ACTION") {
        Some(action_value) => match action_value.as_str() {
            "add" => WorktimeHandler::add(matches),
            _ => panic!("No valid action provided!"),
        },
        None => {
            panic!("No argument provided!");
        }
    }

    Ok(())
}
