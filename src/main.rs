extern crate clap;
use clap::{command, Arg, ArgAction, Command};

mod subcommands;

fn main() {
    let matches = command!()
        .version("0.1.0")
        .author("Dev Myna <var.devmyna@gmail.com>")
        .about("Rust Productivity Manager and Scheduler.")
        .subcommands([
            Command::new("daemon").about("Manage daemon processes").arg(
                Arg::new("ACTION")
                    .help("Action to perform <start|stop>")
                    .required(true)
                    .index(1),
            ),
            Command::new("worktime").about("Manage worktime system."),
            Command::new("init-day").about("Initialize day by setting first tasks of the day."),
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
        .get_matches();

    subcommands::subcommands_matches(&matches);
}
