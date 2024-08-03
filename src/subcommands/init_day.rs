// use chrono::Local;
// use colored::Colorize;
// use std::fs::File;
// use std::io;
// use std::process::Command;

// fn receive_answer() -> String {
//     let mut answer = String::new();
//     io::stdin().read_line(&mut answer).unwrap();

//     answer
// }
// fn get_sleep_task_uuid() -> Vec<u8> {
//     let output = Command::new("task")
//         .args(["due.after:yesterday", "due.before:today", "Sleep", "uuids"])
//         .output();

//     if output.is_ok() {
//         output.unwrap().stdout
//     } else {
//         panic!(
//             "Failed to get sleep task uuids, this is strictly necessary for the init-day command!"
//         );
//     }
// }
// fn get_sleep_awake_date() -> String {
//     let file_path = "/var/tmp/shutdown.cache";

//     #[derive(serde::Deserialize)]
//     struct ShutdownCache {
//         #[serde(rename = "shutdownDate")]
//         shutdown_date: String,
//     }

//     let mut file = File::open(file_path);
//     if file.is_ok() {
//         let mut content = String::new();

//         io::Read::read_to_string(&mut file.unwrap(), &mut content).unwrap();

//         let object = serde_json::from_str::<ShutdownCache>(&content).unwrap();

//         object.shutdown_date
//     } else {
//         panic!("Failed to read sleep init date from cache file!");
//     }
// }

// pub fn init_day() {
//     let current_date = Local::now().format("%Y-%m-%d").to_string();
//     let sleep_task_uuid = get_sleep_task_uuid();
//     let sleep_awake_date = get_sleep_awake_date();

//     println!("{}", "What time did you wake up? (HH:MM)".color("#f1fa8c"));
//     let wake_up_time = receive_answer();

//     // Define sleep logs
//     Command::new("tir")
//         .args([
//             String::from_utf8(sleep_task_uuid).unwrap(),
//             sleep_awake_date,
//             format!("{}T{}", current_date, wake_up_time),
//         ])
//         .spawn()
//         .unwrap();

//     println!(
//         "{}",
//         "What time did you get out of bed? (HH:MM)".color("#f1fa8c")
//     );
//     let get_out_of_bed_time = receive_answer();

//     let current_minutes = Local::now().timestamp() / 60;
// }
