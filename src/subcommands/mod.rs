pub mod daemon;
pub mod init_day;
pub mod task;
pub mod timew;
pub mod worktime;

pub fn subcommands_matches(matches: &clap::ArgMatches) {
    match matches.subcommand() {
        Some(("daemon", matches)) => {
            daemon::init_daemon(matches);
        }
        Some(("worktime", matches)) => {
            worktime::match_action(matches);
        }
        Some(("init-day", _matches)) => {
            init_day::init_day();
        }
        Some(("task", matches)) => {
            task::match_action(matches);
        }
        Some(("timew", matches)) => {
            timew::match_action(matches);
        }
        _ => unreachable!(),
    }
}
