pub mod daemon;
pub mod worktime;
pub mod init_day;
pub mod task;
pub mod timew;

pub fn subcommands_matches(matches: &clap::ArgMatches) {
    match matches.subcommand() {
        Some(("daemon", matches)) => {
            daemon::init_daemon(matches);
        }
        Some(("worktime", _matches)) => {
            todo!();
        }
        Some(("init-day", _matches)) => {
            init_day::init_day();
        }
        Some(("task", _matches)) => {
            task::match_action(matches);
        }
        Some(("timew", matches)) => {
            timew::match_action(matches);
        }
        _ => unreachable!(),
    }
}
