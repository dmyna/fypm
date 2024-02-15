extern crate clap;
use clap::{command, Arg, ArgAction, Command};

mod handlers;
mod subcommands;
mod utils;

fn create_command() -> clap::Command {
    command!()
        .version("0.1.0")
        .author("Dev Myna <var.devmyna@gmail.com>")
        .about("Four Years Productivity Manager.")
        .subcommands([
            Command::new("daemon").about("Manage daemon processes").arg(
                Arg::new("ACTION")
                    .help("Action to perform <start|stop>")
                    .required(true)
                    .index(1),
            ),
            Command::new("worktime")
                .about("Manage worktime system.")
                .args(&[
                    Arg::new("ACTION")
                        .help("Action to perform <add|create|rm|remove|ls|list|apply>")
                        .required(true)
                        .index(1),
                    Arg::new("ACTIONARGS").action(ArgAction::Append).index(2),
                ]),
            Command::new("init-day").about("Initialize day by setting first tasks of the day."),
            Command::new("task")
                .about("Perform taskwarrior actions.")
                .args(&[
                    Arg::new("ACTION")
                        .help("Action to perform <start>")
                        .required(true)
                        .index(1),
                    Arg::new("ACTIONARGS").action(ArgAction::Append).index(2),
                ]),
            Command::new("timew")
                .about("Perform timewarrior actions.")
                .args(&[
                    Arg::new("ACTION")
                        .help("Action to perform <track|start|end|start-correction|end-correction>")
                        .required(true)
                        .index(1),
                    Arg::new("ACTIONARGS").action(ArgAction::Append).index(2),
                ]),
        ])
        .arg_required_else_help(true)
}

fn main() {
    handlers::data_bowl::DataBowlHandler
        .ensure_db_existence()
        .unwrap();

    let matches = create_command().get_matches();

    subcommands::subcommands_matches(&matches);
}
