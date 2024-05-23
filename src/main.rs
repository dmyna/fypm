//#region           Crates
use std::env;
use clap::{Parser, Subcommand};
use lazy_static::lazy_static;
//#endregion
//#region           Modules
mod func;
mod handlers;
mod subcommands;
mod utils;

use func::actions;
use crate::subcommands::{daemon, init_day, instance, task, timew, worktime};
//#endregion
//#region           Structs && Enums
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
    /// Manage instances
    Instance {
        action: String,
        #[arg(long)]
        actionargs: Vec<String>,
    },

    /// Initialize day by setting first tasks of the day
    InitDay,

    /// Anotate on taskwarrior task (tan)
    TaAnnotate { filter: String, annotation: String },
    /// Start a task (tastart)
    TaStart { filter: String },

    /// Anotate on timewarrior task (tin)
    TiAnnotate { filter: String, annotation: String },
    /// Move start of a task to end of other (ticart)
    TiStartCorrection {
        #[arg(default_value_t = String::from("@1"))]
        manipulation_id: String,
        #[arg(default_value_t = String::from("@3"))]
        reference_id: String,
    },
    /// Move end of a task to start of other (ticend)
    TiEndCorrection {
        #[arg(default_value_t = String::from("@3"))]
        manipulation_id: String,
        #[arg(default_value_t = String::from("@1"))]
        reference_id: String,
    },
    /// Move start of a task to end of other (tistart)
    TiStart { id: String, start_time: String },
    /// Move start of a task to end of other (tiend)
    TiEnd { id: String, end_time: String },
    /// Track a task manually (tir)
    TiTrack {
        id: String,
        start_time: String,
        end_time: String,
    },
}
//#endregion
//#region           Constants
lazy_static! {
    static ref DB_PATH: String = env::var("FYPM_DB").unwrap_or_else(|_| dirs::home_dir()
        .unwrap()
        .join(".local/share/fypm")
        .to_string_lossy()
        .into_owned());
}
//#endregion
//#region           Implementation
fn main() {
    handlers::database::DBHandler.ensure_db_existence().unwrap();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Daemon { action, name } => {
            daemon::init_daemon(action, name).unwrap();
        }
        Commands::Worktime { action, actionargs } => {
            worktime::match_action(action, actionargs).unwrap();
        }
        Commands::Instance { action, actionargs } => {
            instance::match_action(action, actionargs).unwrap();
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

        Commands::TiTrack {
            id,
            start_time,
            end_time,
        } => {
            timew::track(id, start_time, end_time).unwrap();
        }
        Commands::TiAnnotate { filter, annotation } => {
            actions::annotate("timew", filter, annotation);
        }
    }
}
//#endregion
