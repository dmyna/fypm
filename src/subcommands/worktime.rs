//#region           Crates
use chrono::NaiveTime;
use dialoguer::{console::Term, Input};
use diesel::Connection;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::SqliteConnection;
use diesel::TextExpressionMethods;
use regex::Regex;
use std::env;
use std::fs;
use std::io::Error;
use std::path::Path;
use std::process::Command;

use crate::db::models::Worktime;
use crate::db::schema::worktimes;
use crate::utils::err::FypmError;
//#endregion
//#region           Modules
use crate::utils::err::FypmErrorKind;
use crate::utils::verify;
use crate::DATABASE_URL;
//#endregion
//#region           Structs
pub struct WorktimeHandler;
//#endregion
//#region           Implementation
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

fn write_values_on_temp_files(
    current_wt: &String,
    bg_poly_color: &String,
    fg_poly_color: &String,
) -> Result<(), Error> {
    let path = Path::new("/var/tmp");
    let last_wt_path = path.join(".last_work_time");

    fs::write(path.join("current_work_time"), current_wt)?;
    fs::write(path.join("current_polybar_b_wt_color"), bg_poly_color)?;
    fs::write(path.join("current_polybar_f_wt_color"), fg_poly_color)?;

    fs::write(last_wt_path, &current_wt)?;

    Ok(())
}

fn update_filter(current_wt: &String, cfg_line: &str) -> Result<(), Error> {
    let current_filter_path = Path::new("/var/tmp/.worktime_filter");
    let config_file_path = dirs::home_dir().unwrap().join(".taskrc");

    let essential_string = "(+TODAY and +INSTANCE)";
    let scheduled_string =
        "((scheduled.after:today or scheduled:today) and scheduled.before:tomorrow)";

    let worktime_filter = format!(
        "(WT:{} or WT:AllDay) and ({} or {})",
        current_wt, essential_string, scheduled_string
    );

    let due_filter = env::var("DUE_FILTER").expect("The DUE_FILTER is not set! Fix it >:(");
    let tw_filter = env::var("TW_FILTER").expect("The TW_FILTER is not set! Fix it >:(");

    let habit_filter = format!(
        "({} and WT:AllDay and (status.not:recurring and status.not:waiting))",
        due_filter
    );

    let main_filter = format!("({} or {}) and status:pending", tw_filter, worktime_filter);

    let final_filter = format!("({}) or {}", main_filter, habit_filter).replace("\\", "");

    let regex = Regex::new(format!("{}.*", cfg_line).as_str()).unwrap();

    let config_file = fs::read_to_string(&config_file_path).unwrap();

    let new_config_file = regex
        .replace_all(&config_file, format!("{}={}", cfg_line, final_filter))
        .to_string();

    fs::write(config_file_path, new_config_file)?;

    fs::write(current_filter_path, final_filter)?;

    Ok(())
}
fn update_viewer_session(viewer: &str, viewer_quit_key: &str) -> Result<(), Error> {
    //. DEV: switch to tmux interface

    let tmux_twui_open = Command::new("tmux")
        .args(["has-session", "-t", "TaskWarrior"])
        .output()?;

    if tmux_twui_open.status.success() {
        Command::new("tmux")
            .args(["send-keys", "-t", "TaskWarrior:0.0", viewer_quit_key, "C-m"])
            .output()?;

        Command::new("tmux")
            .args(["send-keys", "-t", "TaskWarrior:0.0", viewer, "C-m"])
            .output()?;
    } else {
        Command::new("taopen").spawn()?;
    }

    Ok(())
}

pub fn apply(name: &String) -> Result<(), FypmError> {
    let mut conn = SqliteConnection::establish(DATABASE_URL.as_str()).unwrap();

    let get_preset = WorktimeHandler::get(&mut conn, &name);

    match get_preset {
        Ok(preset) => {
            let current_wt_string = format!("{} -> {}", preset.name, preset.end_time.as_str());

            env::set_var("WORKTIME", preset.name);

            write_values_on_temp_files(
                &current_wt_string,
                &preset.polybar_background,
                &preset.polybar_foreground,
            )
            .unwrap();

            {
                let twui_cfg_line = "uda.taskwarrior-tui.task-report.next.filter";

                update_filter(&name, twui_cfg_line).unwrap();
            }

            update_viewer_session("wvit", ":q").unwrap();

            Command::new("polybar-msg").args(["cmd", "restart"]);

            Ok(())
        }
        Err(error) => match error.kind {
            FypmErrorKind::NotFound => {
                println!("{}", error.message);
                println!("These are the available presets:");

                WorktimeHandler::list(&mut conn)?;

                Ok(())
            }
            _ => return Err(error),
        },
    }
}
//#endregion
