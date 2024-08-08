use chrono::{DateTime, Datelike, Duration, Local, NaiveDate, Weekday};
//#region           Crates
use colored::*;
use dialoguer::Confirm;
use std::io::Write;
use std::process::Stdio;
use std::vec;
use std::{fs, process::Command, str};

use crate::func::{command, dialog};
//#endregion
//#region           Modules
use crate::func::{
    action::{self, *},
    list, parser,
};
use crate::handlers::date::NaiveDateIter;
use crate::utils::constants::{CONTROL_TASK, DEFAULT_GET_JSON_OPTIONS, LAST_TASK_PATH};
use crate::utils::enums::{self, TaProjectActions, TaSequenceTypes};
use crate::utils::err::FypmError;
use crate::utils::err::FypmErrorKind;
use crate::utils::structs::TaskWarriorExported;
use crate::utils::{extract, get, term};
//#endregion
//#region           Implementation

pub fn task_stop(
    filter_option: &Option<String>,
    start_control_task: bool,
) -> Result<(), FypmError> {
    let final_filter: String;

    if let Some(filter) = filter_option {
        final_filter = filter.to_string();
    } else {
        let active_tasks = get::get_current_task_json().unwrap();

        final_filter = active_tasks.uuid.to_string();
    }

    Command::new("task")
        .args([&final_filter, "stop"])
        .output()
        .unwrap();

    if start_control_task {
        task_start(&CONTROL_TASK.to_string())?;
    }

    Ok(())
}
pub fn task_start(filter: &String) -> Result<(), FypmError> {
    let mut filter = parser::match_special_aliases(filter);
    let filter_json = get::get_json_by_filter(&filter, DEFAULT_GET_JSON_OPTIONS).unwrap();
    let filter_length = filter_json.len();

    if filter_length == 0 {
        panic!("No task with filter \"{}\" found!", filter);
    } else if filter_length > 1 {
        panic!("Too much tasks with filter \"{}\"!", filter);
    }

    verify_if_wt_is_allday(&filter_json[0]).unwrap();

    verify_if_is_divisory(&filter_json[0]).unwrap();

    filter = match_inforelat_and_sequence(&filter_json[0]).unwrap();

    {
        let active_tasks = get::get_current_task_json();

        if active_tasks.is_err() {
            let err = active_tasks.unwrap_err();

            match err.kind {
                FypmErrorKind::TooMuchTasks => {
                    panic!("There are more than one task active! Fix it >:(.");
                }
                FypmErrorKind::NoTasksFound => {}
                e => {
                    panic!("Unexpected error: {:?}", e);
                }
            }
        } else {
            let active_task_uuid = &active_tasks.unwrap().uuid;
            fs::write(LAST_TASK_PATH, active_task_uuid.as_bytes()).unwrap();

            println!("Stopping active task with uuid: {}", active_task_uuid);
            task_stop(&Some(active_task_uuid.to_string()), false).unwrap();
        }

        println!("Starting task with uuid: {}", filter);
        Command::new("task")
            .args([filter.as_str(), "start"])
            .output()
            .unwrap();

        {
            //. DEV: Implement tascripts in Rust later

            Command::new("tascripts").args([&filter]).output().unwrap();
        }

        Ok(())
    }
}
pub fn task_done(
    filter: &Option<String>,
    tastart_filter: &Option<String>,
    annotation: &Option<String>,
    skip_confirmation: &bool,
    not_necessary: &bool,
    delegated: &bool,
) -> Result<(), FypmError> {
    let mut args = vec!["rc.confirmation=0", "rc.recurrence.confirmation=0"];
    let selected_tasks: Vec<TaskWarriorExported>;

    if let Some(filter) = filter {
        let task_json = get::get_json_by_filter(filter, None)?;

        if let Some(tastart_filter) = tastart_filter {
            task_start(tastart_filter)?;
        } else {
            let current_task = get::get_current_task_json()?;

            for task in &task_json {
                if task.uuid == current_task.uuid {
                    task_start(&CONTROL_TASK.to_string())?;
                    break;
                }
            }
        }

        selected_tasks = task_json;
    } else {
        let current_task = get::get_current_task_json()?;

        if let Some(tastart_filter) = tastart_filter {
            task_start(tastart_filter)?;
        } else {
            task_start(&CONTROL_TASK.to_string())?;
        }

        selected_tasks = vec![current_task];
    }

    let join_uuids = selected_tasks
        .iter()
        .map(|task| task.uuid.as_str())
        .collect::<Vec<&str>>()
        .join(" ");

    args.extend([join_uuids.as_str()]);

    let confirmation: bool;

    if *skip_confirmation {
        confirmation = true;
    } else {
        confirmation = dialog::verify_selected_tasks(&selected_tasks)?;
    }

    if confirmation {
        if let Some(annotation) = annotation {
            action::annotate("task", &join_uuids, annotation, true)?;
        }

        // Tags logic
        {
            let tags: Vec<&str> = vec![];

            if *not_necessary {
                args.push("+NotNecessary");
            }

            if *delegated {
                args.push("+Delegated");
            }

            if tags.len() == 2 {
                return Err(FypmError {
                    message: "You are trying to mark a task with two tags! Are you crazy?"
                        .to_string(),
                    kind: FypmErrorKind::InvalidInput,
                });
            } else if tags.len() == 1 {
                let mut tags_args = args.clone();

                tags_args.push("modify");
                tags_args.extend(tags);

                let mut tag_binding = Command::new("task");
                let tag_command = tag_binding
                    .args(tags_args)
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit());

                let mut tag_child = tag_command.stdin(Stdio::piped()).spawn().unwrap();

                tag_child
                    .stdin
                    .take()
                    .unwrap()
                    .write_all("all\n".as_bytes())
                    .unwrap();
                tag_child.wait().unwrap();
            }
        }
        let mut done_args = args.clone();

        done_args.push("done");

        let mut done_binding = Command::new("task");
        let done_command = done_binding
            .args(done_args)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit());

        if selected_tasks.len() > 2 {
            let mut done_child = done_command.stdin(Stdio::piped()).spawn().unwrap();

            done_child
                .stdin
                .take()
                .unwrap()
                .write_all("all\n".as_bytes())
                .unwrap();
            done_child.wait().unwrap();
        } else {
            done_command.output().unwrap();
        }
    } else {
        println!("Aborting...");
    }

    Ok(())
}
pub fn task_statistic(
    command: &enums::StatisticsCommands,
    no_parents: &bool,
) -> Result<(), FypmError> {
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
pub fn task_add(
    description: &String,
    project: &String,
    style: &String,
    r#type: &String,
    other_args: &Option<Vec<String>>,
    skip_confirmation: &bool,
) -> Result<String, FypmError> {
    if !*skip_confirmation {
        println!("These are the args:");
        println!("      description: {}", description);
        println!("      project: {}", project);
        println!("      STYLE: {}", style);
        println!("      TYPE: {}, ", r#type);
        println!(
            "      others: {}",
            other_args.as_ref().unwrap_or(&vec![]).join(" ")
        );

        let confirmation = Confirm::new()
            .with_prompt("Do you want to continue?")
            .interact()
            .unwrap();

        if !confirmation {
            return Err(FypmError {
                message: "Aborted".to_string(),
                kind: FypmErrorKind::Aborted,
            });
        }
    }

    let mut args = vec![
        "rc.verbose=new-id".to_string(),
        "add".to_string(),
        description.to_string(),
        format!("project:{}", project),
        format!("STYLE:{}", style),
        format!("TYPE:{}", r#type),
    ];

    if let Some(other_args) = other_args {
        args.extend(other_args.clone());
    }

    let execute = Command::new("task").args(args).output();

    let id: String;
    if let Ok(output) = execute {
        if output.status.success() {
            let stdout = str::from_utf8(&output.stdout).unwrap();

            id = stdout
                .trim()
                .replace("Created task ", "")
                .replace(".", "")
                .to_string();
        } else {
            panic!(
                "An error occurred trying to create a task: {}",
                str::from_utf8(&output.stderr).unwrap()
            );
        }
    } else {
        let error = execute.unwrap_err();

        panic!("An error occurred trying to create a task: {}", error);
    }

    let uuid = get::get_uuids_by_filter(&id, DEFAULT_GET_JSON_OPTIONS)?[0].clone();

    println!("Created task with id \"{}\"! ({})", id, uuid);

    Ok(uuid)
}
pub fn task_add_sub(
    mother_task: &String,
    other_args: &Vec<String>,
    skip_confirmation: &bool,
) -> Result<String, FypmError> {
    let subtask: String;

    let get_mother_task_json = get::get_json_by_filter(mother_task, DEFAULT_GET_JSON_OPTIONS)?;
    let mother_task_json = get_mother_task_json.get(0).unwrap();

    if other_args.len() == 1 {
        subtask = other_args.get(0).unwrap().clone();

        let get_subtask_uuid = get::get_uuids_by_filter(&subtask, DEFAULT_GET_JSON_OPTIONS)?;
        let subtask_uuid = get_subtask_uuid.get(0).unwrap();

        Command::new("task")
            .args([subtask_uuid.as_str(), "modify", "+SUBTASK"])
            .output()
            .unwrap();
    } else if other_args.len() >= 3 {
        let project: &String;
        let description = other_args.get(0).unwrap();
        let style = other_args.get(1).unwrap();
        let r#type = other_args.get(2).unwrap();

        if let Some(project_arg) = &mother_task_json.project {
            project = project_arg;
        } else {
            panic!("The specified mother doesn't have a project setted... Are you writing this stuff right?");
        }

        let args = other_args.get(3..).unwrap().to_vec();

        let uuid = task_add(
            description,
            project,
            style,
            r#type,
            &Some(args),
            skip_confirmation,
        )?;

        subtask = uuid;
    } else {
        return Err(FypmError {
            message: "You specified a wrong number of arguments! You don't know how to read documentation, do you? :P".to_string(),
            kind: FypmErrorKind::InvalidInput
        });
    }

    Command::new("task")
        .args([mother_task.as_str(), "modify", "STATE:Info", "+MOTHER"])
        .output()
        .unwrap();
    println!("Mother task setted.");

    Command::new("task")
        .args([
            &subtask,
            &"modify".to_string(),
            &format!("MOTHER:{}", mother_task_json.uuid),
            &"+SUBTASK".to_string(),
        ])
        .output()
        .unwrap();
    println!(
        "Subtask added to its MOTHER '{}'!",
        mother_task_json.description
    );

    Ok(subtask)
}
pub fn task_add_seq(
    seq_type: &TaSequenceTypes,
    style: &String,
    description: &String,
    project: &String,
    tag: &String,
    initial_number: &usize,
    last_number: &usize,
    season: &Option<String>,
    last_season_id: &Option<String>,
) -> Result<(), FypmError> {
    let mother_task_uuid: String;
    let mother_description: String;
    let final_tag = format!("+ST_{}", tag);
    let final_tag_type = format!("+{}", seq_type.to_string());

    if let Some(season) = season {
        mother_description = format!("{} (Season {})", description, season)
    } else {
        mother_description = format!("{}", description);
    }

    {
        let uuid = task_add(
            &mother_description,
            &project.to_string(),
            &style,
            &"Objective".to_string(),
            &Some(vec![
                "+Sequence".to_string(),
                final_tag.clone(),
                final_tag_type.clone(),
            ]),
            &true,
        )?;

        mother_task_uuid = uuid;
    }

    let mut previous_task_uuid: String = "".to_string();

    for i in *initial_number..=*last_number {
        let mother_task_uuid = &mother_task_uuid;
        let subtask_description: String;

        match seq_type {
            TaSequenceTypes::Book => {
                subtask_description = format!("Chapter {}", i);
            }
            _ => {
                if let Some(season) = season {
                    subtask_description = format!("S{}E{}", season, i);
                } else {
                    subtask_description = format!("E{}", i);
                }
            }
        }

        let mut args = vec![
            subtask_description.clone(),
            style.clone(),
            final_tag.clone(),
            final_tag_type.clone(),
            "+Sequence".to_string(),
        ];

        if i == *initial_number {
            if let Some(last_season_id) = last_season_id {
                let get_last_season_json =
                    get::get_json_by_filter(&last_season_id, DEFAULT_GET_JSON_OPTIONS).unwrap();
                let last_season_json = get_last_season_json.get(0).unwrap();

                args.push(format!("SEQ_PREVIOUS:{}", last_season_json.uuid));
            }

            let current_task_uuid = task_add_sub(&mother_task_uuid, &args, &true).unwrap();

            if let Some(last_season_id) = last_season_id {
                Command::new("task")
                    .args([
                        last_season_id,
                        &"modify".to_string(),
                        &format!("SEQ_PREVIOUS:{}", current_task_uuid),
                    ])
                    .output()
                    .unwrap();
            }

            Command::new("task")
                .args([
                    mother_task_uuid,
                    &"modify".to_string(),
                    &format!("SEQ_CURRENT:{}", current_task_uuid),
                ])
                .output()
                .unwrap();

            previous_task_uuid = current_task_uuid;
        } else {
            if previous_task_uuid == "".to_string() {
                panic!("previous_task_uuid is empty!");
            }

            let current_task_uuid = task_add_sub(&mother_task_uuid, &args, &true).unwrap();

            Command::new("task")
                .args([
                    &current_task_uuid,
                    &"modify".to_string(),
                    &format!("SEQ_PREVIOUS:{}", previous_task_uuid),
                ])
                .output()
                .unwrap();
            Command::new("task")
                .args([
                    previous_task_uuid,
                    "modify".to_string(),
                    format!("SEQ_NEXT:{}", &current_task_uuid),
                ])
                .output()
                .unwrap();

            previous_task_uuid = current_task_uuid;
        }
    }

    Ok(())
}
pub fn task_add_brth(birthday_person: &String, date: &String) -> Result<String, FypmError> {
    let current_year = Local::now().year().to_string();

    let date =
        DateTime::parse_from_rfc3339(format!("{}-{}T23:59:59Z", current_year, date).as_str())
            .unwrap()
            .date_naive();

    let current_date = Local::now().date_naive();

    let mut final_date: String = "".to_string();

    if current_date <= date {
        final_date = date.to_string();
    } else {
        let add_a_year = date.with_year(date.year() + 1);

        if let Some(new_date) = add_a_year {
            final_date = new_date.to_string();
        }
    }

    let uuid = task_add(
        &format!("{}'s Birthday", birthday_person),
        &"Social.Events".to_string(),
        &"Dionysian".to_string(),
        &"Event".to_string(),
        &Some(vec![
            "WT:AllDay!".to_string(),
            "recur:yearly".to_string(),
            format!("GOAL:{}T00:00:00", &final_date),
            format!("due:{}T23:59:59", &final_date),
        ]),
        &true,
    )?;

    Ok(uuid)
    //Ok(uuid)
}
pub fn task_add_pl(playlist_name: &String, length: &u16) -> Result<String, FypmError> {
    let style = "Dionysian".to_string();

    let mother_uuid = task_add(
        &playlist_name,
        &"Music.Playlist".to_string(),
        &style,
        &"Objective".to_string(),
        &None,
        &true,
    )?;

    task_add_sub(
        &mother_uuid,
        &vec!["Cover".to_string(), style.clone()],
        &true,
    )?;
    task_add_sub(
        &mother_uuid,
        &vec!["Description".to_string(), style.clone()],
        &true,
    )?;
    task_add_sub(
        &mother_uuid,
        &vec![format!("Songs ({})", length), style],
        &true,
    )?;

    Ok(mother_uuid)
}

pub fn task_list_date(
    property: &String,
    modifier: &String,
    date_args: &Vec<String>,
) -> Result<(), FypmError> {
    let verbose: &str = "rc.verbose=0";
    let sort = format!("rc.report.{modifier}.sort={property}");
    let divisory = "                            ";

    let initial_date: NaiveDate;
    let final_date: NaiveDate;

    [initial_date, final_date] = extract::date_period(date_args);

    for date in NaiveDateIter::new(initial_date, final_date) {
        let initial_day = date.format("%Y-%m-%d").to_string();
        let final_day = (date + Duration::days(1)).format("%Y-%m-%d").to_string();

        Command::new("task")
            .args([
                format!("{verbose}"), format!("{sort}"),
                format!("({property}.after:{initial_day} or {property}:{initial_day}) and {property}.before:{final_day}"),
                format!("{modifier}")])
            .stdout(Stdio::inherit())
            .output()
            .unwrap();

        if date.weekday() == Weekday::Sun {
            println!("{divisory}");
        }
    }

    Ok(())
}
pub fn task_list_mother_and_subtasks(
    modifier: &String,
    filter: &Vec<String>,
) -> Result<(), FypmError> {
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
pub fn list_completion_score(date_args: &Vec<String>) -> Result<(), FypmError> {
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

        let tasks_json = get::get_json_by_filter(format!("((due.after:{initial_day} or due:{initial_day}) and due.before:{final_day}) and +INSTANCE").as_str(), None)?;

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

pub fn task_abandon(
    tag: &enums::TaAbandonTags,
    filter: &String,
    annotation: &Option<String>,
) -> Result<(), FypmError> {
    if (tag == &enums::TaAbandonTags::Abandoned || tag == &enums::TaAbandonTags::NoControl)
        && annotation.is_none()
    {
        panic!("You must specify an annotation when mark a task as NoControl or Abandoned!");
    }
    let tasks = get::get_json_by_filter(filter, None)?;
    let tasks_count: usize = tasks.len();
    let confirmation = dialog::verify_selected_tasks(&tasks)?;

    if confirmation {
        let mut modify_args = Vec::new();
        modify_args.extend([
            "rc.verbose=0",
            "rc.recurrence.confirmation=0",
            "rc.confirmation=0",
            filter,
            "modify",
        ]);

        match tag {
            enums::TaAbandonTags::Archived => {
                modify_args.extend(["+Archived"]);
            }
            enums::TaAbandonTags::Failed => {
                modify_args.extend(["+Failed"]);
            }
            enums::TaAbandonTags::Abandoned => {
                modify_args.extend(["+Abandoned"]);
            }
            enums::TaAbandonTags::NoControl => {
                modify_args.extend(["+NoControl"]);
            }
        }

        if let Some(annotation) = annotation {
            action::annotate("task", filter, annotation, true)?;
        }

        let mut modify_binding = Command::new("task");
        let modify_command = modify_binding.args(modify_args).stderr(Stdio::inherit());

        let mut delete_binding = Command::new("task");
        let delete_command = delete_binding
            .args([
                "rc.verbose=0",
                "rc.confirmation=0",
                "rc.recurrence.confirmation=0",
                filter,
                "delete",
            ])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit());

        if tasks_count > 2 {
            command::stdin_all(modify_command).unwrap();

            command::stdin_all(delete_command).unwrap();
        } else {
            modify_command.output().unwrap();

            delete_command.output().unwrap();
        }
    } else {
        println!("Aborting...");
    }

    Ok(())
}
pub fn task_schedule(
    filter: &String,
    alarm_date: &String,
    due_date: &Option<String>,
    worktime: &Option<String>,
) -> Result<(), FypmError> {
    let tasks = get::get_json_by_filter(filter, None)?;
    let tasks_count: usize = tasks.len();
    let confirmation = dialog::verify_selected_tasks(&tasks)?;

    if confirmation {
        let mut modify_args = Vec::new();
        modify_args.extend([
            "rc.verbose=0".to_string(),
            "rc.recurrence.confirmation=0".to_string(),
            "rc.confirmation=0".to_string(),
            filter.clone(),
            "modify".to_string(),
        ]);

        if alarm_date != "cur" {
            modify_args.extend([format!("ALARM:{}", alarm_date)]);
        }
        if let Some(due_date) = due_date {
            modify_args.extend([format!("due:{}", due_date)]);
        }
        if let Some(worktime) = worktime {
            modify_args.extend([format!("WT:{}", worktime)]);
        }

        let mut modify_binding = Command::new("task");
        let modify_command = modify_binding
            .args(modify_args)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit());

        if tasks_count > 2 {
            let mut modify_child = modify_command.stdin(Stdio::piped()).spawn().unwrap();

            modify_child
                .stdin
                .take()
                .unwrap()
                .write_all("all\n".as_bytes())
                .unwrap();
            modify_child.wait().unwrap();
        } else {
            modify_command.output().unwrap();
        }
    } else {
        println!("Aborting...");
    }

    Ok(())
}
pub fn task_unschedule(
    filter: &String,
    no_alarm: &bool,
    no_due: &bool,
    no_worktime: &bool,
) -> Result<(), FypmError> {
    let tasks = get::get_json_by_filter(filter, None)?;
    let tasks_count: usize = tasks.len();
    let confirmation = dialog::verify_selected_tasks(&tasks)?;

    if confirmation {
        let mut modify_args = Vec::new();
        modify_args.extend([
            "rc.verbose=0",
            "rc.recurrence.confirmation=0",
            "rc.confirmation=0",
            filter,
            "modify",
        ]);

        if !*no_alarm {
            modify_args.extend(["ALARM:"]);
        }
        if !*no_due {
            modify_args.extend(["due:"]);
        }
        if !*no_worktime {
            modify_args.extend(["WT:NonSched!"]);
        }

        let mut modify_binding = Command::new("task");
        let modify_command = modify_binding
            .args(modify_args)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit());

        if tasks_count > 2 {
            let mut modify_child = modify_command.stdin(Stdio::piped()).spawn().unwrap();

            modify_child
                .stdin
                .take()
                .unwrap()
                .write_all("all\n".as_bytes())
                .unwrap();
            modify_child.wait().unwrap();
        } else {
            modify_command.output().unwrap();
        }
    } else {
        println!("Aborting...");
    }

    Ok(())
}
pub fn task_und(filter: &String, unarchive: &bool) -> Result<(), FypmError> {
    let tasks = if *unarchive {
        println!("Unarchive option is true! Filtering for archived tasks...");

        get::get_json_by_filter(format!("(+Archived and ({}))", filter).as_str(), None)?
    } else {
        get::get_json_by_filter(filter, None)?
    };

    let confirmation = dialog::verify_selected_tasks(&tasks)?;

    if confirmation {
        let mut args = vec![
            "rc.verbose=0",
            "rc.confirmation=0",
            "rc.recurrence.confirmation=0",
            filter,
            "modify",
            "status:pending",
        ];

        if *unarchive {
            action::unarchive(tasks)?;

            return Ok(());
        } else {
            args.extend(["-Failed", "-Abandoned", "-NoControl"]);
        }

        let mut modify_binding = Command::new("task");
        let modify_command = modify_binding.args(args).stderr(Stdio::inherit());

        if tasks.len() > 2 {
            command::stdin_all(modify_command).unwrap();
        } else {
            modify_command.output().unwrap();
        }
    } else {
        println!("Aborting...");
    }

    Ok(())
}
pub fn task_project(action: &TaProjectActions, arg: &Option<String>) -> Result<(), FypmError> {
    let no_project_specified = FypmError {
        message: "Please provide a project name!".to_string(),
        kind: FypmErrorKind::InvalidInput,
    };

    match *action {
        TaProjectActions::List => {
            let mut args = Vec::new();

            if let Some(filter) = arg {
                args.extend([format!("project:{}", filter)]);
            }

            args.extend(["projects".to_string()]);

            Command::new("task")
                .args(args)
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .output()
                .unwrap();
        }
        TaProjectActions::Add => {
            if let Some(project) = arg {
                let confirmation: bool = Confirm::new()
                    .with_prompt(format!("Do you want to add '{}' project?", project))
                    .interact()
                    .unwrap();

                if confirmation {
                    task_add(
                        &"Project Marker".to_string(),
                        project,
                        &" ".to_string(),
                        &"Continuous".to_string(),
                        &Some(vec!["+FYPM".to_string()]),
                        &true,
                    )?;
                }
            } else {
                return Err(no_project_specified);
            }
        }
        TaProjectActions::Archive => {
            if let Some(project) = arg {
                let confirmation: bool = Confirm::new()
                    .with_prompt(format!("Do you want to archive '{}' project?", project))
                    .interact()
                    .unwrap();

                if confirmation {
                    task_abandon(
                        &enums::TaAbandonTags::Archived,
                        &format!("(project:{} and -DELETED and -COMPLETED)", project),
                        &None,
                    )?;
                }
            } else {
                return Err(no_project_specified);
            }
        }
        TaProjectActions::Unarchive => {
            if let Some(project) = arg {
                let confirmation: bool = Confirm::new()
                    .with_prompt(format!("Do you want to unarchive '{}' project?", project))
                    .interact()
                    .unwrap();

                if confirmation {
                    println!("Unarchive option is true! Filtering for archived tasks...");

                    let tasks: Vec<TaskWarriorExported> = get::get_json_by_filter(
                        format!("(project:{} and +Archived)", project).as_str(),
                        None,
                    )?;

                    action::unarchive(tasks)?;
                }
            } else {
                return Err(no_project_specified);
            }
        }
    }

    Ok(())
}
//#endregion
