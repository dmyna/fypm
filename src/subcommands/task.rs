use crate::utils::paths;
use crate::utils::structs;
use std::str;
use std::{fs, process::Command};

fn task_start(id: &str, schedule: Option<bool>) {
    let last_task_path = "/tmp/.last_task";
    let get_last_task = fs::read(last_task_path).expect("I can't get the last task :(");
    let last_task = String::from_utf8(get_last_task).expect("I can't parse the last_task vec!");

    let get_active_tasks = Command::new("task")
        .args(["+ACTIVE", "export"])
        .output()
        .expect("It's not possible to get active tasks!");

    let active_tasks: Vec<structs::TaskWarriorExported> = serde_json::from_str(
        str::from_utf8(&get_active_tasks.stdout).expect("I can't parse stdout!"),
    )
    .expect("Failed to parse received json!");

    fn lock(active: bool) -> () {
        let lock_path = format!("{}/task.lock", paths::get_paths().tmp_dir);

        if active {
            fs::write::<String, &str>(lock_path, "").unwrap();
        } else {
            fs::remove_file(lock_path).unwrap();
        }
    }

    lock(true);
}

pub fn match_action(matches: &clap::ArgMatches) {
    match matches.get_one::<String>("ACTION") {
        Some(action_value) => match action_value.as_str() {
            // "start" => {
            //     task_start()
            // }
            _ => panic!("No valid action provided!"),
        },
        None => {
            panic!("No argument provided!");
        }
    }
}
