use crate::func::action;
use crate::subcommands::{daemon, init_day, instance, task, timew, worktime};
use crate::utils::enums::{Commands, TimewAction};
use crate::utils::err::FypmError;

pub fn match_subcommand(command: &Commands) -> Result<(), FypmError> {
    match command {
        Commands::InitDay => todo!(),
        Commands::Daemon { action, name } => todo!(),
        Commands::Worktime { action, actionargs } => todo!(),
        Commands::Instance { action, actionargs } => todo!(),


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
