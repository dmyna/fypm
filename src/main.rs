use clap::{Parser, Subcommand};

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
        #[arg(long)]
        action: String,
        #[arg(long)]
        name: String,
    },
    /// Manage worktime system
    Worktime {
        #[arg(long)]
        action: String,
        #[arg(long)]
        actionargs: Vec<String>,
    },
    /// Initialize day by setting first tasks of the day
    InitDay,
    /// Perform taskwarrior actions
    Task {
        #[arg(long)]
        action: String,
        #[arg(long)]
        actionargs: Vec<String>,
    },
    /// Perform timew actions
    Timew {
        #[arg(long)]
        action: String,
        #[arg(long)]
        actionargs: Vec<String>,
    },
}

fn main() {
    handlers::data_bowl::DataBowlHandler
        .ensure_db_existence()
        .unwrap();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Daemon { action, name } => {
            daemon::init_daemon(action, name);
        }
        Commands::Worktime { action, actionargs } => {
            worktime::match_action(action, actionargs);
        }
        Commands::InitDay => {
            init_day::init_day();
        }
        Commands::Task { action, actionargs } => {
            task::match_action(action, actionargs);
        }
        Commands::Timew { action, actionargs } => {
            timew::match_action(action, actionargs);
        }
    }
}
