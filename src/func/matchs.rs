use crate::func::action;
use crate::subcommands::{task, timew};
use crate::utils::enums::{self, Commands, TimewAction};
use crate::utils::err::FypmError;
use std::io::Error;
use std::str;

pub fn match_subcommand(command: &Commands) -> Result<(), FypmError> {
    match command {
        Commands::InitDay => todo!(),
        Commands::Daemon { action, name } => todo!(),
        Commands::Worktime { action, actionargs } => todo!(),
        Commands::Instance { action, actionargs } => todo!(),

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
                &false,
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
            let execute =
                task::task_add_sub(mother_task, other_args, skip_confirmation, &false).unwrap();

            match execute {
                _ => Ok(()),
            }
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

        Commands::TaStart { filter } => task::task_start(filter),
        Commands::TaStop { filter } => task::task_stop(filter, true),
        Commands::TaDone {
            tasks_to_done,
            tastart_filter,
        } => task::task_done(tasks_to_done, tastart_filter),
        Commands::TaAnnotate { filter, annotation } => action::annotate("task", filter, annotation),
        Commands::TaStatistic { name, no_parents } => task::task_statistic(name, no_parents),

        Commands::TiEndCorrection {
            manipulation_id,
            reference_id,
        } => timew::time_move(&TimewAction::End, manipulation_id, reference_id),
        Commands::TiStartCorrection {
            manipulation_id,
            reference_id,
        } => timew::time_move(&TimewAction::Start, manipulation_id, reference_id),
        Commands::TiStart { id, start_time } => {
            timew::time_set(&TimewAction::Start, id, start_time)
        }
        Commands::TiEnd { id, end_time } => timew::time_set(&TimewAction::End, id, end_time),
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
            action::annotate("timew", filter, annotation)
        }
    }
}
pub fn match_exec_command(
    executed_command: Result<std::process::Output, Error>,
) -> Result<(), Error> {
    match executed_command {
        Ok(output) => {
            if output.status.success() {
                println!("{}", str::from_utf8(&output.stdout).unwrap());
            } else {
                eprintln!("{}", str::from_utf8(&output.stderr).unwrap());
            }

            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to execute command, error: {}", e);

            Err(e)
        }
    }
}
