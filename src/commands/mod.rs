pub mod init_day;
pub mod task;
pub mod timew;
pub mod worktime;
pub mod instance;


use diesel::SqliteConnection;
use diesel::Connection;

use crate::api;
use crate::handlers::aliases;
use crate::handlers::filters::FiltersHandler;
use crate::handlers::worktime::WorktimeHandler;
use crate::values::enums;
use crate::values::enums::AliasActions;
use crate::values::enums::{Commands, TimewAction};
use crate::values::err::FypmError;
use crate::{func, DATABASE_URL};

pub async fn matching(command: &Commands) -> Result<(), FypmError> {
    match command {
        //#region               Misc
        Commands::Completion => func::completion::generate_completion(),
        //#endregion
        //#region               Systems
        Commands::Daemon => {
            api::index::rocket().launch().await.unwrap();

            Ok(())
        },
        Commands::InitDay => todo!(),

        Commands::Verify { script } => func::matchs::match_verify_script(script),

        Commands::Alias { action, filter } => {
            match action {
                AliasActions::Add => aliases::AliasesHandler::add(filter),
                AliasActions::Change => {
                    //aliases::AliasesHandler::change(filter)
                    todo!()
                }
            }
        }

        Commands::Filter { action } => {
            let conn = &mut SqliteConnection::establish(DATABASE_URL.as_str()).unwrap();

            match action {
                enums::FilterActions::Add => FiltersHandler::add(conn),
                enums::FilterActions::List => FiltersHandler::list(conn),
                enums::FilterActions::Remove => FiltersHandler::remove(conn),
                enums::FilterActions::Edit => FiltersHandler::edit(conn),
            }
        },

        Commands::WtAdd { worktime_name } => {
            WorktimeHandler::add(
                &mut SqliteConnection::establish(DATABASE_URL.as_str()).unwrap(),
                worktime_name,
            )?;

            Ok(())
        }
        Commands::WtRemove { worktime_name } => {
            WorktimeHandler::remove(
                &mut SqliteConnection::establish(DATABASE_URL.as_str()).unwrap(),
                worktime_name,
            )?;

            Ok(())
        }
        Commands::WtLs => {
            WorktimeHandler::list(
                &mut SqliteConnection::establish(DATABASE_URL.as_str()).unwrap(),
            )?;

            Ok(())
        }
        Commands::WtApply { worktime_name } => {
            worktime::apply(worktime_name)?;

            Ok(())
        }

        Commands::Instance { action, actionargs } => instance::match_action(action, actionargs),
        //#endregion
        //#region               Task Subcommands
        Commands::TaInfo {
            filter,
        } => task::list::info(filter),

        Commands::TaAdd {
            description,
            project,
            style,
            r#type,
            other_args,
            skip_confirmation,
        } => {
            let execute = task::add::new(
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
            task::add::subtask(mother_task, other_args, skip_confirmation)?;

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
        } => task::add::sequence(
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
            task::add::birthday(birthday_person, date)?;

            Ok(())
        }
        Commands::TaAddPl {
            playlist_name,
            length,
        } => {
            task::add::playlist(playlist_name, length)?;

            Ok(())
        }

        Commands::TaLsDate {
            property,
            modifier,
            date_args,
        } => {
            if let Some(date_args) = date_args {
                task::list::date(property, modifier, date_args)?;
            } else {
                task::list::date(property, modifier, &vec!["-w".to_string()])?;
            }

            Ok(())
        }
        Commands::TaLsMotAndSub { modifier, filter } => {
            task::list::mother_and_subtasks(modifier, filter)?;

            Ok(())
        }
        Commands::TaLsScore { date_args } => {
            if let Some(date_args) = date_args {
                task::list::completion_score(date_args).unwrap();
            } else {
                task::list::completion_score(&vec!["-w".to_string()]).unwrap();
            }

            Ok(())
        }

        Commands::TaStart { filter } => task::update::start(filter),
        Commands::TaStop { filter } => task::update::stop(filter, true),
        Commands::TaDone {
            tasks_to_done,
            tastart_filter,
            annotation,
            skip_confirmation,
            not_necessary,
            delegated,
        } => task::update::done(
            tasks_to_done,
            tastart_filter,
            annotation,
            skip_confirmation,
            not_necessary,
            delegated,
        ),
        Commands::TaAnnotate { filter, annotation } => {
            func::action::annotate("task", filter, annotation, false)
        }
        Commands::TaAbandon {
            tag,
            filter,
            annotation,
            annotation_filter,
        } => task::update::abandon(tag, filter, annotation, annotation_filter),
        Commands::TaStatistic { name, no_parents } => task::list::statistic(name, no_parents),
        Commands::TaSchedule {
            filter,
            alarm_date,
            due_date,
            worktime,
        } => task::update::schedule(filter, alarm_date, due_date, worktime),
        Commands::TaUnschedule {
            filter,
            no_alarm,
            no_due,
            no_worktime,
        } => task::update::unschedule(filter, no_alarm, no_due, no_worktime),
        Commands::TaUnd { filter, unarchive } => task::update::und(filter, unarchive),
        Commands::TaRecurTime { filter, new_time } => {
            task::update::recur_time(filter, new_time)
        }
        Commands::TaProject { action, arg } => task::task_project(action, arg),
        //#endregion
        //#region               Timew Subcommands
        Commands::TiLs { initial_date, final_date } => {
            timew::list(initial_date, final_date)?;

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
            args
        } => timew::track(id, args),
        Commands::TiReplace {
            original_id,
            replacement_id,
        } => timew::replace(original_id, replacement_id),
        Commands::TiAnnotate { filter, annotation } => {
            func::action::annotate("timew", filter, annotation, false)
        } //#endregion
    }
}