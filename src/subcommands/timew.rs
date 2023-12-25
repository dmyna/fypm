use std::process::Command;
use std::str;
#[derive(serde::Deserialize)]
#[allow(unused)]
struct TaskWarriorExported {
    id: i32,
    #[serde(rename = "STATE")]
    state: String,
    #[serde(rename = "STYLE")]
    style: String,
    #[serde(rename = "TYPE")]
    r#type: String,
    #[serde(rename = "WT")]
    wt: String,
    description: String,
    entry: String,
    modified: String,
    project: String,
    status: String,
    uuid: String,
    tags: Option<Vec<String>>,
    urgency: f64,
}
#[derive(serde::Deserialize)]
struct TimeWarriorExported {
    id: i32,
    start: String,
    end: Option<String>,
    tags: Option<Vec<String>>,
}

pub fn timew_end_correction(matches: &clap::ArgMatches) {
    fn usage() {
        println!("Usage: ticart [tomodify_task_end](default: @3) [static_task_start](default: @1)");
    }

    let received_args = matches.get_many::<String>("ACTIONARGS").unwrap();

    fn receive_start_time(start_received_id: Option<String>) -> String {
        let mut start_id = String::new();

        if let Some(id) = start_received_id {
            start_id = id;
        } else {
            start_id = "@1".to_string();
        }

        let get_task_info = Command::new("timew")
            .args([start_id.as_str(), "export"])
            .output()
            .expect("Failed to get timew json!");

        let str_json = str::from_utf8(&get_task_info.stdout).unwrap();
        let tasks_json: Vec<TimeWarriorExported> =
            serde_json::from_str(str_json).expect("Failed to parse received json!");

        let task_json = tasks_json.get(0).unwrap();

        return task_json.start.clone();
    }

    let mut end_id = String::new();

    if let Some(end_task_id) = received_args.clone().nth(0) {
        end_id = end_task_id.clone();
    } else {
        end_id = "@3".to_string();
    }

    if received_args.len() > 2 {
        eprint!("Hey... You don't need to specify more than two parameters!");
        usage();
    }

    timew_end(
        end_id.as_str(),
        receive_start_time(received_args.clone().nth(1).cloned()).as_str(),
    );
}
pub fn timew_end(received_id: &str, received_end_time: &str) {
    if !received_id.starts_with("@") {
        panic!("Hey!! Are you trying to use a taskwarrior id? Specify with \"@\"!");
    }

    let execute = Command::new("timew")
        .args(["modify", "end", received_id, received_end_time, ":adjust"])
        .output();

    match execute {
        Ok(output) => {
            println!("{}", str::from_utf8(&output.stdout).unwrap());
        }
        Err(e) => eprintln!("Failed to execute timew command, error: {}", e),
    }
}
pub fn timew_track(matches: &clap::ArgMatches) {
    let max_description_length = 25;

    let received_args = matches.get_many::<String>("ACTIONARGS").unwrap();
    let received_id = received_args
        .clone()
        .nth(0)
        .expect("No task id provided!")
        .as_str();

    let received_start_time = received_args
        .clone()
        .nth(1)
        .expect("No task start time provided!")
        .as_str();
    let receved_final_time = received_args
        .clone()
        .nth(2)
        .expect("No task final time provided!")
        .as_str();

    let get_task_info = Command::new("task")
        .args([received_id, "export"])
        .output()
        .expect("Failed to get task json!");

    let str_json = str::from_utf8(&get_task_info.stdout).unwrap();
    let tasks_json: Vec<TaskWarriorExported> =
        serde_json::from_str(str_json).expect("Failed to parse received json!");

    let task_json = tasks_json.get(0).unwrap();

    let trucated_description = format!(
        "{}...",
        &task_json.description[..max_description_length - 3]
    );

    let mut args = vec![
        "track",
        received_start_time,
        "-",
        receved_final_time,
        task_json.uuid.as_str(),
        trucated_description.as_str(),
        task_json.project.as_str(),
        task_json.style.as_str(),
        task_json.r#type.as_str(),
        task_json.wt.as_str(),
        ":adjust",
    ];

    if let Some(tags) = &task_json.tags {
        args.extend(tags.iter().map(|tag| tag.as_str()));
    }

    let execute = Command::new("timew").args(&args).output();

    match execute {
        Ok(output) => {
            println!("{}", str::from_utf8(&output.stdout).unwrap());
        }
        Err(e) => eprintln!("Failed to execute timew command, error: {}", e),
    }
}
pub fn match_action(matches: &clap::ArgMatches) {
    match matches.get_one::<String>("ACTION") {
        Some(action_value) => match action_value.as_str() {
            "end-correction" => {
                timew_end_correction(matches);
            }
            "end" => {
                fn usage() {
                    println!("Usage: timew end <task id> <task end time>");
                }

                let received_args = matches.get_many::<String>("ACTIONARGS").unwrap();

                if received_args.len() == 1 {
                    eprintln!("Do you not specify a task time?");
                } else if received_args.len() == 0 {
                    eprintln!("You need to specifiy something! >:(");
                    usage();
                } else if received_args.len() > 2 {
                    eprint!("Hey... You don't need to specify more than");
                    eprint!("two parameters... Is there an error here?");
                    usage();
                } else {
                    timew_end(
                        received_args.clone().nth(0).unwrap(),
                        received_args.clone().nth(1).unwrap(),
                    );
                }
            }
            "track" => {
                timew_track(matches);
            }
            _ => panic!("No valid action provided!"),
        },
        None => {
            panic!("No argument provided!");
        }
    }
}
