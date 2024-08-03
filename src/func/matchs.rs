use chrono::NaiveDate;
use rusqlite::Connection;

use crate::func::action;
use crate::func::date;
use crate::handlers;
use crate::handlers::aliases;
use crate::subcommands::instance;
use crate::subcommands::worktime;
use crate::subcommands::worktime::WorktimeHandler;
use crate::subcommands::{task, timew};
use crate::utils::enums;
use crate::utils::enums::AliasActions;
use crate::utils::enums::{Commands, TimewAction};
use crate::utils::err::FypmError;
use crate::MAIN_DB_FILE;

pub fn match_subcommand(command: &Commands) -> Result<(), FypmError> {
    match command {
        //#region               Systems
        Commands::InitDay => todo!(),

        Commands::Verify { script } => match_verify_script(script),

        Commands::Alias { action, filter } => {
            match action {
                AliasActions::Add => aliases::AliasesHandler::add(filter),
                AliasActions::Change => {
                    //aliases::AliasesHandler::change(filter)
                    todo!()
                }
            }
        }

        Commands::WtAdd { worktime_name } => {
            WorktimeHandler {
                conn: Connection::open(MAIN_DB_FILE.to_string()).unwrap().into(),
            }
            .add(worktime_name)?;

            Ok(())
        }
        Commands::WtRemove { worktime_name } => {
            WorktimeHandler {
                conn: Connection::open(MAIN_DB_FILE.to_string()).unwrap().into(),
            }
            .remove(worktime_name)?;

            Ok(())
        }
        Commands::WtLs => {
            WorktimeHandler {
                conn: Connection::open(MAIN_DB_FILE.to_string()).unwrap().into(),
            }
            .list()?;

            Ok(())
        }
        Commands::WtApply { worktime_name } => {
            worktime::apply(worktime_name)?;

            Ok(())
        }

        Commands::Instance { action, actionargs } => instance::match_action(action, actionargs),
        //#endregion
        //#region               Task Subcommands
        Commands::TaAdd {
            description,
            project,
            style,
            r#type,
            other_args,
            skip_confirmation,
        } => {
            let execute = task::task_add(
                description,
                project,
                style,
                r#type,
                other_args,
                skip_confirmation,
            );

            match execute.unwrap() {
                _ => Ok(()),
            }
        }
        Commands::TaAddSub {
            mother_task,
            other_args,
            skip_confirmation,
        } => {
            task::task_add_sub(mother_task, other_args, skip_confirmation)?;

            Ok(())
        }
        Commands::TaAddSeq {
            seq_type,
            style,
            description,
            project,
            tag,
            initial_number,
            last_number,
            season,
            last_season_id,
        } => task::task_add_seq(
            seq_type,
            style,
            description,
            project,
            tag,
            initial_number,
            last_number,
            season,
            last_season_id,
        ),
        Commands::TaAddBrth {
            birthday_person,
            date,
        } => {
            task::task_add_brth(birthday_person, date)?;

            Ok(())
        }
        Commands::TaAddPl {
            playlist_name,
            length,
        } => {
            task::task_add_pl(playlist_name, length)?;

            Ok(())
        }

        Commands::TaLsDate {
            property,
            modifier,
            date_args,
        } => {
            if let Some(date_args) = date_args {
                task::task_list_date(property, modifier, date_args)?;
            } else {
                task::task_list_date(property, modifier, &vec!["-w".to_string()])?;
            }

            Ok(())
        }
        Commands::TaLsMotAndSub { modifier, filter } => {
            task::task_list_mother_and_subtasks(modifier, filter)?;

            Ok(())
        }
        Commands::TaLsScore { date_args } => {
            if let Some(date_args) = date_args {
                task::list_completion_score(date_args).unwrap();
            } else {
                task::list_completion_score(&vec!["-w".to_string()]).unwrap();
            }

            Ok(())
        }

        Commands::TaStart { filter } => task::task_start(filter),
        Commands::TaStop { filter } => task::task_stop(filter, true),
        Commands::TaDone {
            tasks_to_done,
            tastart_filter,
        } => task::task_done(tasks_to_done, tastart_filter),
        Commands::TaAnnotate { filter, annotation } => {
            action::annotate("task", filter, annotation, false)
        }
        Commands::TaAbandon {
            tag,
            filter,
            annotation,
        } => task::task_abandon(tag, filter, annotation),
        Commands::TaStatistic { name, no_parents } => task::task_statistic(name, no_parents),
        Commands::TaSchedule {
            filter,
            alarm_date,
            due_date,
            worktime,
        } => task::task_schedule(filter, alarm_date, due_date, worktime),
        Commands::TaUnschedule {
            filter,
            no_alarm,
            no_due,
            no_worktime,
        } => task::task_unschedule(filter, no_alarm, no_due, no_worktime),
        Commands::TaUnd { filter, unarchive } => task::task_und(filter, unarchive),
        Commands::TaProject { action, arg } => task::task_project(action, arg),
        //#endregion
        //#region               Timew Subcommands
        Commands::TiLs { date, filters } => {
            timew::list(date, filters)?;

            Ok(())
        }
        Commands::TiEndCorrection {
            manipulation_id,
            reference_id,
        } => timew::move_log(&TimewAction::End, manipulation_id, reference_id),
        Commands::TiStartCorrection {
            manipulation_id,
            reference_id,
        } => timew::move_log(&TimewAction::Start, manipulation_id, reference_id),
        Commands::TiStart { id, start_time } => timew::set_log(&TimewAction::Start, id, start_time),
        Commands::TiEnd { id, end_time } => timew::set_log(&TimewAction::End, id, end_time),
        Commands::TiTrack {
            id,
            start_time,
            end_time,
        } => timew::track(id, start_time, end_time),
        Commands::TiReplace {
            original_id,
            replacement_id,
        } => timew::replace(original_id, replacement_id),
        Commands::TiAnnotate { filter, annotation } => {
            action::annotate("timew", filter, annotation, false)
        } //#endregion
    }
}
// pub fn match_exec_command(
//     executed_command: Result<std::process::Output, Error>,
// ) -> Result<(), Error> {
//     match executed_command {
//         Ok(output) => {
//             if output.status.success() {
//                 println!("{}", str::from_utf8(&output.stdout).unwrap());
//             } else {
//                 eprintln!("{}", str::from_utf8(&output.stderr).unwrap());
//             }

//             Ok(())
//         }
//         Err(e) => {
//             eprintln!("Failed to execute command, error: {}", e);

//             Err(e)
//         }
//     }
// }
pub fn match_date_arg(option: &String, option_arg: Option<&String>) -> [NaiveDate; 2] {
    match option.as_str() {
        "-y" | "--year" => date::get_year(option_arg),
        "-m" | "--month" => date::get_month(option_arg),
        "-w" | "--week" => date::get_week(option_arg),
        _ => {
            panic!("You entered an invalid option to date_args!");
        }
    }
}

pub fn match_verify_script(script: &enums::VerifyScripts) -> Result<(), FypmError> {
    match script {
        enums::VerifyScripts::Aliases => handlers::aliases::verify_aliases_tasks(),
    }
}
