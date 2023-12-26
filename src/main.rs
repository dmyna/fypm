extern crate clap;
use clap::{command, Arg, ArgAction, Command};

mod subcommands;

fn main() {
    let matches = command!()
        .author("Dev Myna <var.devmyna@gmail.com>")
        .about("A manager of my workflow and the better programs that I use.")
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
        .get_matches();

    subcommands::subcommands_matches(&matches);
}
