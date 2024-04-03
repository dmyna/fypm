//#region           Crates
use chrono::NaiveTime;
use dialoguer::{console::Term, Input};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::path::Path;
use std::process::Command;

//#endregion
//#region           Modules
use crate::handlers::data_bowl::{DataBowlHandler, PresetHandler};
use crate::utils::verify;
//#endregion
//#region           Constants
const DATA_BOWL_NAME: &str = "worktime";
lazy_static! {
    static ref WORKTIME_DATABOWL_PATH: String = {
        dirs::home_dir()
            .unwrap()
            .join(".local/share/fypm")
            .join(DATA_BOWL_NAME)
            .to_string_lossy()
            .into_owned()
    };
}
//#endregion
//#region           Structs
#[derive(Debug, Serialize, Deserialize)]
struct Worktime {
    style: String,
    start_time: String,
    end_time: String,
    polybar_background: String,
    polybar_foreground: String,
}
pub struct WorktimeHandler;
//#endregion
//#region           Implementation
impl WorktimeHandler {
    pub fn ensure_worktime_data_bowl() {
        let verify_existence =
            DataBowlHandler::verify_by_name(&DATA_BOWL_NAME.to_string()).unwrap();

        if verify_existence == false {
            DataBowlHandler::create(
                &DATA_BOWL_NAME.to_string(),
                &String::from("The databowl of your worktimes!"),
            )
            .unwrap();
        }
    }
    pub fn add(args: &Vec<String>) -> Result<(), Error> {
        let date_format = "%H:%M";
        let term = Term::stdout();

        if args.len() != 1 {
            if args.len() > 1 {
                panic!("Too much arguments!");
            } else {
                panic!("Not enough arguments!");
            }
        }

        let name = args.get(0).unwrap().to_string();

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
            style,
            start_time,
            end_time,
            polybar_background,
            polybar_foreground,
        };

        let preset_instance = PresetHandler {
            data_bowl_name: DATA_BOWL_NAME.to_string(),
        };

        preset_instance.add::<Worktime>(&name, &description, &new_worktime)
    }
    pub fn remove(args: &Vec<String>) -> Result<(), Error> {
        if args.len() != 1 {
            if args.len() > 1 {
                panic!("Too much arguments!");
            } else {
                panic!("Not enough arguments!");
            }
        }

        let name = args.get(0).unwrap().to_string();

        let preset_instance = PresetHandler {
            data_bowl_name: DATA_BOWL_NAME.to_string(),
        };

        preset_instance.remove(&name)?;

        println!("Removed preset {}!", name);

        Ok(())
    }
    pub fn list() -> Result<(), Error> {
        let preset_instance = PresetHandler {
            data_bowl_name: DATA_BOWL_NAME.to_string(),
        };

        let worktimes = preset_instance.list()?;

        if worktimes.is_empty() {
            println!("No worktimes found!");
        } else {
            println!("Found {} worktimes. These are:", worktimes.len());

            for worktime in worktimes {
                println!("{} - {}", worktime.0, worktime.1);
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

    fs::write(path.join("current_work_time"), current_wt)?;
    fs::write(path.join("current_polybar_b_wt_color"), bg_poly_color)?;
    fs::write(path.join("current_polybar_f_wt_color"), fg_poly_color)?;

    Ok(())
}
fn update_vit_taskrc() {
    let cfg_file_path = dirs::home_dir().unwrap().join(".taskrc");
    let vit_cfg_file_path = dirs::home_dir().unwrap().join(".vit_taskrc");

    let cfg_file = fs::read_to_string(&cfg_file_path).unwrap();

    let reader = BufReader::new(cfg_file.as_bytes());

    let check_comment = |line: &Result<String, Error>| {
        let line = line.as_ref().ok();

        if let Some(line) = line {
            !line.starts_with("#NO-VIT")
        } else {
            false
        }
    };

    let lines = reader.lines();

    let new_lines: Vec<String> = lines
        .take_while(check_comment)
        .map(|line| line.unwrap())
        .collect();

    let new_vit_cfg_file = new_lines.join("\n");

    fs::write(vit_cfg_file_path, new_vit_cfg_file).unwrap();
}

fn update_filter(current_wt: &String, cfg_line: &str) -> () {
    let config_file_path = dirs::home_dir().unwrap().join(".taskrc");

    // !DEV
    // transform this: "(GOAL.after:$(date +%Y-%m-01) and GOAL.before:now and TYPE:Objective)"

    let goal_string = "(GOAL.before:now and due.before:eom and TYPE:Objective)";
    let essential_string = "(+TODAY and +INSTANCE)";
    let scheduled_string =
        "((scheduled.after:today or scheduled:today) and scheduled.before:tomorrow)";

    let worktime_filter = format!(
        "(WT:{} or WT:AllDay) and ({} or {} or {})",
        current_wt, essential_string, goal_string, scheduled_string
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

    fs::write(config_file_path, new_config_file).unwrap();

    update_vit_taskrc();
}

pub fn apply(args: &Vec<String>) -> Result<(), Error> {
    if args.len() != 1 {
        if args.len() > 1 {
            panic!("Too much arguments!");
        } else {
            panic!("Not enough arguments!");
        }
    }

    let name = args.get(0).unwrap().to_string();

    let preset_instance = PresetHandler {
        data_bowl_name: DATA_BOWL_NAME.to_string(),
    };

    let preset = preset_instance.get(&name).unwrap();

    let preset_params = toml::from_str::<Worktime>(preset.params.as_str()).unwrap();

    let current_wt_string = format!("{} -> {}", preset.name, preset_params.end_time.as_str());

    env::set_var("WORKTIME", preset.name);

    write_values_on_temp_files(
        &current_wt_string,
        &preset_params.polybar_background,
        &preset_params.polybar_foreground,
    )
    .unwrap();

    {
        let vit_cfg_line = "report.wlist.filter";
        let twui_cfg_line = "uda.taskwarrior-tui.task-report.next.filter";

        update_filter(&name, vit_cfg_line);
        update_filter(&name, twui_cfg_line);
    }
    {
        let viewer = "twui";
        let viewer_quit_key = "q";

        let tmux_twui_open = Command::new("tmux")
            .args(["has-session", "-t", "TaskWarrior"])
            .output()
            .unwrap();

        if tmux_twui_open.status.success() {
            Command::new("tmux")
                .args(["send-keys", "-t", "TaskWarrior:0.0", viewer_quit_key, "C-m"])
                .output()
                .unwrap();

            Command::new("tmux")
                .args(["send-keys", "-t", "TaskWarrior:0.0", viewer, "C-m"])
                .output()
                .unwrap();
        } else {
            Command::new("taopen").spawn().unwrap();
        }
    }

    Command::new("polybar-msg").args(["cmd", "restart"]);

    Ok(())
}

pub fn match_action(action: &String, actionargs: &Vec<String>) -> Result<(), Error> {
    WorktimeHandler::ensure_worktime_data_bowl();

    match action.as_str() {
        "add" | "create" => WorktimeHandler::add(actionargs).unwrap(),
        "rm" | "remove" => WorktimeHandler::remove(actionargs).unwrap(),
        "ls" | "list" => WorktimeHandler::list().unwrap(),
        "apply" => apply(actionargs).unwrap(),
        _ => panic!("No valid action provided!"),
    }

    Ok(())
}
//#endregion
