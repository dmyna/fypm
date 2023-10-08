extern crate daemonize

use std::fs;
use std::thread;
use std::time::Duration;
use daemonize::Daemonize;

pub fn daemon(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let tmp_file = "/tmp/rpms_daemon";

    let action = matches.value_of("ACTION").unwrap();
    let daemon_name = matches.value_of("NAME").unwrap();

    struct Actions;
    impl Actions {
        pub fn start() {
            let create_daemon_tmp = fs::create_dir_all(tmp_file).unwrap()?;
            let stdout = fs::File::create(format!("{}{}_stdout.log", tmp_file, daemon_name))?;
            let stderr = fs::File::create(format!("{}{}_stderr.log", tmp_file, daemon_name))?;

            let daemonize = Daemonize::new()
                .pid_file(format!("/tmp/rpms_daemon_{}.pid", daemon_name))
                .working_directory("~/")
                .stdout(stdout)
                .stderr(stderr)

        }
        pub fn kill() {

        }
    }

    let action = match action {
        "start" => Actions::start(),
        "stop" => Actions::kill(),
        _ => unreachable!(),
    };

    Ok(())
}
