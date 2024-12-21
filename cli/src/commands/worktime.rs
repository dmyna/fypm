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

//#region           Crates
use diesel::Connection;
use diesel::SqliteConnection;
use regex::Regex;
use std::env;
use std::fs;
use std::io::Error;
use std::path::Path;
use std::process::Command;

use crate::handlers::worktime::WorktimeHandler;
use fypm_lib::values::err::FypmError;
//#endregion
//#region           Modules
use fypm_lib::values::err::FypmErrorKind;
use crate::DATABASE_URL;
//#endregion
//#endregion
//#region           Implementation

/// Writes the current worktime and associated polybar colors to temporary files.
///
/// This function creates or updates temporary files in the `/var/tmp` directory
/// to store the current worktime and its corresponding polybar background and
/// foreground colors. It also updates the `.last_work_time` file with the current worktime.
///
/// # Arguments
///
/// * `current_wt` - A reference to a string containing the name of the current worktime.
/// * `bg_poly_color` - A reference to a string containing the polybar background color for the worktime.
/// * `fg_poly_color` - A reference to a string containing the polybar foreground color for the worktime.
///
/// # Returns
///
/// * `Result<(), Error>` - Returns an `Ok` result if writing to the files is successful,
///   otherwise returns an `Error`.
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
/// Updates the filter for the worktime command in the `~/.taskrc` file.
///
/// The worktime filter is created by combining the essential and scheduled filters. The
/// essential filter is defined by the `ESSENTIAL_FILTER` env variable and the scheduled
/// filter is defined by the `SCHEDULED_FILTER` env variable. The worktime filter is then
/// used to filter the tasks and update the `~/.taskrc` file.
///
/// # Arguments
///
/// * `current_wt` - The current worktime that is selected.
/// * `cfg_line` - The line of the config file that needs to be updated.
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
/// Updates the TaskWarrior session to use the specified viewer and quit key.
///
/// If the TaskWarrior session exists, it will quit the current viewer and open the specified one.
/// If the TaskWarrior session does not exist, it will simply open the specified viewer.
///
/// # Arguments
///
/// * `viewer` - The viewer to be used.
/// * `viewer_quit_key` - The quit key for the viewer.
///
/// # Returns
///
/// A Result containing either Ok(()) if the viewer was updated successfully, or an Error if an error occurred.
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
/// Applies a worktime preset to the system.
///
/// The function works by setting the WORKTIME environment variable, writing the current worktime
/// and its associated Polybar colors to temporary files, setting the next filter in the
/// taskwarrior-tui configuration and restarting the Polybar instance.
///
/// If the preset is not found, the function prints a message and lists the available presets.
///
/// # Errors
///
/// The function will return an error if the preset is not found in the database.
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
