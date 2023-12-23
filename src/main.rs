extern crate clap;
use clap::{command, Arg, Command};

mod subcommands;

fn main() {
    let matches = command!()
        .author("Dev Myna <var.devmyna@gmail.com>")
        .about("A manager of my workflow and the better programs that I use.")
        .subcommand(
            Command::new("daemon").about("Manage daemon processes").arg(
                Arg::new("ACTION")
                    .help("Action to perform <start|stop>")
                    .required(true)
                    .index(1),
            ),
        )
        .subcommand(
            Command::new("worktime").about("Manage worktime system."),
        )
        .subcommand(
            Command::new("init-day").about("Initialize day by setting first tasks of the day."),
        )
        .subcommand(
            Command::new("timew").about("Perform timewarrior actions."),
        )
        .get_matches();

    subcommands::subcommands_matches(&matches);
}
