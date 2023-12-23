pub mod daemon;
pub mod worktime;
pub mod init_day;
pub mod timew;

pub fn subcommands_matches(matches: &clap::ArgMatches) {
    match matches.subcommand() {
        Some(("daemon", matches)) => {
            daemon::init_daemon(matches);
        }
        Some(("worktime", _matches)) => {
            worktime::worktime();
        }
        Some(("init-day", _matches)) => {
            init_day::init_day();
        }
        Some(("timew", matches)) => {
            timew::match_action(matches);
        },
        _ => unreachable!(),
    }
}
