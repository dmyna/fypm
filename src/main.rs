use clap::{Parser, Subcommand};
use func::actions;

mod func;
mod handlers;
mod subcommands;
mod utils;

use crate::subcommands::{daemon, init_day, task, timew, worktime};

#[derive(Parser)]
#[command(name = "fypm")]
#[command(version = "0.1.0")]
#[command(about = "Four Years Productivity Manager", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
pub enum Commands {
    /// Manage daemon processes
    Daemon {
        action: String,
        #[arg(long)]
        name: String,
    },
    /// Manage worktime system
    Worktime {
        action: String,
        #[arg(long)]
        actionargs: Vec<String>,
    },
    /// Initialize day by setting first tasks of the day
    InitDay,

    /// Anotate on taskwarrior task
    TaAnnotate { filter: String, annotation: String },
    /// Start a task
    TaStart { filter: String },

    /// Anotate on timewarrior task
    TiAnnotate { filter: String, annotation: String },
    /// Move start of a task to end of other
    TiStartCorrection {
        #[arg(default_value_t = String::from("@1"))]
        manipulation_id: String,
        #[arg(default_value_t = String::from("@3"))]
        reference_id: String,
    },
    /// Move end of a task to start of other
    TiEndCorrection {
        #[arg(default_value_t = String::from("@3"))]
        manipulation_id: String,
        #[arg(default_value_t = String::from("@1"))]
        reference_id: String,
    },
    /// Move start of a task to end of other
    TiStart { id: String, start_time: String },
    /// Move start of a task to end of other
    TiEnd { id: String, end_time: String },
    /// Track a task manually
    TiTrack {
        id: String,
        start_time: String,
        end_time: String,
    }
}

fn main() {
    handlers::data_bowl::DataBowlHandler
        .ensure_db_existence()
        .unwrap();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Daemon { action, name } => {
            daemon::init_daemon(action, name).unwrap();
        }
        Commands::Worktime { action, actionargs } => {
            worktime::match_action(action, actionargs).unwrap();
        }
        Commands::InitDay => {
            init_day::init_day();
        }

        Commands::TaStart { filter } => {
            task::task_start(filter);
        }
        Commands::TaAnnotate { filter, annotation } => {
            actions::annotate("task", filter, annotation);
        }

        Commands::TiEndCorrection {
            manipulation_id,
            reference_id,
        } => {
            timew::time_move(
                &timew::TimewAction::End,
                vec![manipulation_id, reference_id],
            )
            .unwrap();
        }
        Commands::TiStartCorrection {
            manipulation_id,
            reference_id,
        } => {
            timew::time_move(
                &timew::TimewAction::Start,
                vec![manipulation_id, reference_id],
            )
            .unwrap();
        }
        Commands::TiStart { id, start_time } => {
            timew::time_set(&timew::TimewAction::End, id, start_time).unwrap();
        }
        Commands::TiEnd { id, end_time } => {
            timew::time_set(&timew::TimewAction::End, id, end_time).unwrap();
        }

        Commands::TiTrack { id, start_time, end_time  } => {

            timew::track(id, start_time, end_time).unwrap();
        }
        Commands::TiAnnotate { filter, annotation } => {
            actions::annotate("timew", filter, annotation);
        }
    }
}
