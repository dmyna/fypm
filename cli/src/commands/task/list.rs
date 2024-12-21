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

use chrono::{Datelike, Local};
use chrono::{Duration, NaiveDate, Weekday};
use colored::*;
use std::process::{Command, Stdio};

use fypm_lib::utils::parser;
use fypm_lib::values::constants::DEFAULT_GET_JSON_OPTIONS;
use fypm_lib::values::err::{FypmErrorKind, FypmError};
use fypm_lib::values::structs::TaskWarriorStatus;

use crate::commands;
use crate::{
    func::list,
    handlers::date::NaiveDateIter,
    utils::{get, term},
};

/// Displays detailed information about a specific task based on the given filter.
///
/// The function fetches task data using the provided filter and formats it into
/// a structured table, displaying key task attributes such as project, style,
/// work time, type, quadrant, effort, and tags. The first column contains
/// task identifiers and attributes, while the second column contains their
/// corresponding values. The table is styled for readability, with alternating
/// row colors and emphasis on the task identifier.
///
/// # Arguments
///
/// * `filter` - A string representing the filter criteria for retrieving the task.
///
/// # Returns
///
/// * `Result<(), FypmError>` - Returns an empty result if successful, or a
///   `FypmError` if an error occurs during data retrieval or processing.
///
/// # Errors
///
/// * `FypmErrorKind::ProblemWithStoredTask` - If there is a mismatch in the
///   number of attributes and values, indicating an issue with task data.
pub fn info(filter: &String) -> Result<(), FypmError> {
    let grid_separator_len = 1;
    let task = get::json_by_filter(filter, DEFAULT_GET_JSON_OPTIONS)?;

    let mut left_lines: Vec<String> = vec![];
    let mut right_lines: Vec<String> = vec![];

    let mut left_values = vec![];
    let mut right_values = vec![];

    {
        if task[0].id == 0 {
            left_values.push(task[0].uuid.chars().take(8).collect::<String>());
        } else {
            left_values.push(task[0].id.to_string());
        }

        left_values.extend(vec![
            "Project".to_string(),
            "Style".to_string(),
            "WorkTime".to_string(),
            "Type".to_string(),
            "Quadrant".to_string(),
            "Effort".to_string(),
            "".to_string(),
            "Tags".to_string(),
        ]);

        right_values.extend(vec![
            task[0].description.clone(),
            task[0].project.clone().unwrap_or("".to_string()),
            task[0].style.clone().unwrap_or("".to_string()),
            task[0].wt.clone(),
            task[0].r#type.clone(),
            task[0].quadrant.clone().unwrap_or("".to_string()),
            task[0].effort.clone().unwrap_or("".to_string()),
            "".to_string(),
        ]);

        if let Some(tags) = &task[0].tags {
            right_values.push(tags.join(", "));
        } else {
            right_values.push("".to_string());
        }
    }

    let left_side_len = left_values.iter().map(|s| s.chars().count()).max().unwrap();
    let right_side_len = right_values
        .iter()
        .map(|s| s.chars().count())
        .max()
        .unwrap();

    for i in 0..(left_values.len()) {
        let left_value = &left_values[i];
        let right_value = &right_values[i];

        let left_spaces = ' '
            .to_string()
            .repeat(left_side_len - left_value.len() + grid_separator_len);
        let right_spaces = ' '.to_string().repeat(right_side_len - right_value.len());

        left_lines.push(format!("{}{}", left_value, left_spaces));
        right_lines.push(format!("{}{}", right_value, right_spaces));
    }

    if left_lines.len() != right_lines.len() {
        return Err(FypmError {
            message: "Left values and right values have different length!".to_string(),
            kind: FypmErrorKind::ProblemWithStoredTask,
        });
    }

    let max_len = left_lines[0].len() + right_lines[0].len();

    // Pretty print the table
    {
        let bg_color = Color::TrueColor {
            r: 40,
            g: 40,
            b: 40,
        };

        for i in 0..(left_values.len()) {
            if i == 0 {
                print!("{}", left_lines[i].italic().bold());
                println!("{}", right_lines[i]);
                println!("{}", " ".to_string().repeat(max_len).underline())
            } else if i % 2 == 0 {
                print!("{}", left_lines[i].on_color(bg_color));
                println!("{}", right_lines[i].on_color(bg_color));
            } else {
                print!("{}", left_lines[i]);
                println!("{}", right_lines[i]);
            }
        }
    }

    Ok(())
}

pub fn statistic(command: &commands::StatisticsCommands, no_parents: &bool) -> Result<(), FypmError> {
    match command {
        commands::StatisticsCommands::Deleted => {
            list::deleted_tasks(no_parents)?;
        }
        commands::StatisticsCommands::Pending => {
            list::pending_tasks(no_parents)?;
        }
    }

    Ok(())
}
/// Lists tasks by date within a specified range, applying sorting and filtering.
///
/// For each day in the range, tasks are retrieved and listed based on the
/// specified property and modifier. The sorting is controlled by a format
/// string, and the results are printed to standard output, with each day
/// visually separated. Sundays are marked with an additional visual separator.
///
/// # Arguments
///
/// * `property` - A reference to a string that specifies which property to
///   consider for task sorting and filtering.
/// * `modifier` - A reference to a string that specifies additional filter
///   criteria to apply when retrieving tasks.
/// * `date_args` - A vector of strings representing the start and end dates
///   for the range, formatted as `YYYY-MM-DD`.
///
/// # Returns
///
/// * `Result<(), FypmError>` - Returns an Ok result if successful, or a
///   `FypmError` if an error occurs during task retrieval or processing.
///
/// # Errors
///
/// * `FypmErrorKind::TaskWarriorError` - If the command execution fails during
///   task retrieval or counting.
///
/// # Output
///
/// The function outputs the tasks to the standard output, grouped by dates
/// within the specified range, and prints them using a bold and white format.
pub fn date(
    property: &String,
    modifier: &String,
    date_args: &Vec<String>,
) -> Result<(), FypmError> {
    let verbose: &str = "rc.verbose:label";
    let sort = format!("rc.report.{modifier}.sort={property}");

    let initial_date: NaiveDate;
    let final_date: NaiveDate;

    [initial_date, final_date] = parser::date_period(date_args);

    for date in NaiveDateIter::new(initial_date, final_date) {
        let initial_day = date.format("%Y-%m-%d").to_string();
        let final_day = (date + Duration::days(1)).format("%Y-%m-%d").to_string();

        println!(
            "{}",
            date.format("%a - %Y-%m-%d")
                .to_string()
                .bright_white()
                .bold()
        );
        term::print_full_divisory();

        Command::new("task")
            .args([
                format!("{verbose}"), format!("{sort}"),
                format!("({property}.after:{initial_day} or {property}:{initial_day}) and {property}.before:{final_day}"),
                format!("{modifier}")])
            .stdout(Stdio::inherit())
            .output()
            .unwrap();
        println!();

        if date.weekday() == Weekday::Sun {
            term::print_full_divisory();
        }
    }

    Ok(())
}

/// Lists tasks with mother tasks and their subtasks grouped together.
///
/// This function filters tasks based on the provided modifier and filter criteria.
/// It first retrieves the UUIDs of mother tasks and lists them with their
/// subtasks, applying the specified modifier. It then lists additional tasks
/// that match the filter but are not mother tasks.
///
/// # Arguments
///
/// * `modifier` - A reference to a string that specifies the modifier for listing tasks.
/// * `filter` - A reference to a vector of strings that holds additional filter criteria.
///
/// # Returns
///
/// * `Result<(), FypmError>` - Returns an Ok result if successful, or a `FypmError` if an error occurs.
///
/// # Errors
///
/// * `FypmErrorKind::TaskWarriorError` - If the command execution fails during task retrieval or counting.
///
/// # Output
///
/// The function outputs the tasks to the standard output, grouped by mothers and subtasks,
/// and prints the total number of tasks found.
pub fn mother_and_subtasks(modifier: &String, filter: &Vec<String>) -> Result<(), FypmError> {
    let modifier_filter: String;

    let mut tasks_count = 0;

    if modifier != "all" {
        modifier_filter = get::filter_by_modifier(modifier)?
    } else {
        modifier_filter = "".to_string();
    }

    let other_tasks_filter = &format!(
        "((({}) {}) and MOTHER: and -MOTHER)",
        filter.join(" "),
        modifier_filter
    );

    {
        let mothers_uuids = get::get_uuids_by_filter(
            format!("(({}) and +MOTHER)", filter.join(" ")).as_str(),
            None,
        )?;

        for mother_uuid in mothers_uuids {
            let tasks_filter =
                format!("((uuid:{mother_uuid} or MOTHER:{mother_uuid}) {modifier_filter})");

            tasks_count += get::get_count_by_filter(&tasks_filter)?;

            Command::new("task")
                .args([
                    tasks_filter.as_str(),
                    "rc.verbose=0",
                    "rc.urgency.user.tag.MOTHER.coefficient=1100",
                    format!("rc.report.{modifier}.sort=urgency-").as_str(),
                    format!("{modifier}").as_str(),
                ])
                .stdout(Stdio::inherit())
                .output()
                .unwrap();
        }

        tasks_count += get::get_count_by_filter(other_tasks_filter)?;
    }

    {
        Command::new("task")
            .args([
                other_tasks_filter,
                "rc.verbose=0",
                format!("rc.report.{modifier}.sort=TYPE-,entry+").as_str(),
                modifier,
            ])
            .stdout(Stdio::inherit())
            .output()
            .unwrap();

        println!();

        term::print_full_divisory();

        println!();

        println!("{} tasks found", tasks_count);
    }

    Ok(())
}
/// Prints a report of the completion score for each day in the given range.
///
/// The report will show the number of tasks completed, pending, and deleted for
/// each day, as well as the total number of tasks. For each day, it will also
/// show the percentage of tasks completed, pending, and deleted.
///
/// If the current date is within the given range, it will be highlighted in
/// the report.
///
/// The report will also show a summary of the completion score for the week.
/// The summary will show the total number of tasks completed, pending, and
/// deleted for the week, as well as the percentage of tasks completed, pending,
/// and deleted.
pub fn completion_score(date_args: &Vec<String>) -> Result<(), FypmError> {
    let initial_date: NaiveDate;
    let final_date: NaiveDate;

    [initial_date, final_date] = parser::date_period(date_args);

    let mut week_pending = 0;
    let mut week_completed = 0;
    let mut week_deleted = 0;
    let mut week_total = 0;
    for date in NaiveDateIter::new(initial_date, final_date) {
        let initial_day = date.format("%Y-%m-%d").to_string();
        let final_day = (date + Duration::days(1)).format("%Y-%m-%d").to_string();

        let tasks_json = get::json_by_filter(format!("((due.after:{initial_day} or due:{initial_day}) and due.before:{final_day}) and +INSTANCE").as_str(), None)?;

        let pending_count = tasks_json
            .iter()
            .filter(|task| task.status == TaskWarriorStatus::Pending)
            .count();
        week_pending += pending_count;

        let completed_count = tasks_json
            .iter()
            .filter(|task| task.status == TaskWarriorStatus::Completed)
            .count();
        week_completed += completed_count;

        let deleted_count = tasks_json
            .iter()
            .filter(|task| task.status == TaskWarriorStatus::Deleted)
            .count();
        week_deleted += deleted_count;

        let total_count = pending_count + completed_count + deleted_count;
        week_total += total_count;

        let no_pend_count = total_count - pending_count;

        if total_count == 0 {
            continue;
        }

        {
            println!(
                "{}: {} {} {}",
                initial_day.bold(),
                pending_count.to_string().cyan(),
                completed_count.to_string().bright_green(),
                deleted_count.to_string().bright_red()
            );

            if pending_count > 0 {
                print!(
                    "              - ({} / {}) -> {}%",
                    no_pend_count.to_string().bright_black(),
                    total_count.to_string().bright_black(),
                    ((pending_count * 100) / total_count)
                        .to_string()
                        .bright_black(),
                );
            } else {
                print!(
                    "              - ({}) ->",
                    total_count.to_string().bright_black(),
                );
            }

            print!(
                " {}% {}%",
                ((completed_count * 100) / total_count)
                    .to_string()
                    .bright_green(),
                ((deleted_count * 100) / total_count)
                    .to_string()
                    .bright_red()
            );

            if date == Local::now().date_naive() {
                print!(
                    "                    {}",
                    "<───── TODAY".bright_white().bold()
                );
            }

            println!();
            println!();

            if date.weekday() == Weekday::Sun || date == final_date {
                term::print_full_divisory();

                println!(
                    "{}: {} {} {}",
                    "Week Status".to_string().bold(),
                    week_pending.to_string().cyan(),
                    week_completed.to_string().bright_green(),
                    week_deleted.to_string().bright_red(),
                );

                if week_pending > 0 {
                    print!(
                        "              - ({} / {}) ->",
                        (week_total - week_pending).to_string().bright_black(),
                        week_total.to_string().bright_black(),
                    );
                } else {
                    print!(
                        "              - ({}) ->",
                        week_total.to_string().bright_black()
                    );
                }

                print!(
                    " {}% {}%\n",
                    ((week_completed * 100) / week_total)
                        .to_string()
                        .bright_green(),
                    ((week_deleted * 100) / week_total).to_string().bright_red()
                );

                week_pending = 0;
                week_completed = 0;
                week_deleted = 0;
                week_total = 0;

                term::print_full_divisory();
            }
        }
    }

    Ok(())
}
