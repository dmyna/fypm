use daemonize::Daemonize;
use std::fs;

pub fn init_daemon(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let tmp_file_name = "/tmp/rpms_daemon";

    let action = *matches.get_one::<&str>("ACTION").unwrap();
    let daemon_name = *matches.get_one::<&str>("NAME").unwrap();

    struct Actions;
    impl Actions {
        pub fn start(tmp_file_name: &str, daemon_name: &str) {
            let create_daemon_tmp = fs::create_dir_all(&tmp_file_name);
            let stdout = fs::File::create(format!("{}{}_stdout.log", &tmp_file_name, &daemon_name));
            let stderr = fs::File::create(format!("{}{}_stderr.log", &tmp_file_name, &daemon_name));

            if create_daemon_tmp.is_ok() && stdout.is_ok() && stderr.is_ok() {
                let daemonize = Daemonize::new()
                    .pid_file(format!("/tmp/rpms_daemon_{}.pid", daemon_name))
                    .working_directory("~/")
                    .stdout(stdout.unwrap())
                    .stderr(stderr.unwrap());
            }
        }
        pub fn kill() {
            todo!();
        }
    }

    let action = match action {
        "start" => Actions::start(tmp_file_name, daemon_name),
        "stop" => Actions::kill(),
        _ => unreachable!(),
    };

    Ok(())
}
