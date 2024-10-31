use std::process::{Command, Stdio};
use colored::*;
use chrono::{Datelike, Local};
use chrono::{Duration, NaiveDate, Weekday};

use crate::{func::list, handlers::date::NaiveDateIter, utils::{extract, get, term}, values::{enums, err::FypmError}};

pub fn statistic(command: &enums::StatisticsCommands, no_parents: &bool) -> Result<(), FypmError> {
    match command {
        enums::StatisticsCommands::Deleted => {
            list::deleted_tasks(no_parents)?;
        }
        enums::StatisticsCommands::Pending => {
            list::pending_tasks(no_parents)?;
        }
    }

    Ok(())
}
pub fn date(
    property: &String,
    modifier: &String,
    date_args: &Vec<String>,
) -> Result<(), FypmError> {
    let verbose: &str = "rc.verbose:label";
    let sort = format!("rc.report.{modifier}.sort={property}");

    let initial_date: NaiveDate;
    let final_date: NaiveDate;

    [initial_date, final_date] = extract::date_period(date_args);

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
pub fn completion_score(date_args: &Vec<String>) -> Result<(), FypmError> {
    let initial_date: NaiveDate;
    let final_date: NaiveDate;

    [initial_date, final_date] = extract::date_period(date_args);

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
            .filter(|task| task.status.as_str() == "pending")
            .count();
        week_pending += pending_count;

        let completed_count = tasks_json
            .iter()
            .filter(|task| task.status.as_str() == "completed")
            .count();
        week_completed += completed_count;

        let deleted_count = tasks_json
            .iter()
            .filter(|task| task.status.as_str() == "deleted")
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
