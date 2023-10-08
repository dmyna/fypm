extern crate clap;
use clap::{command, Arg, Command};

use crate::subcommands::*;

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
        .get_matches();

    match matches.subcommand() {
        Some(("daemon", matches)) => {
            daemon(matches);
        }
    }
}
