use dialoguer::Confirm;

use fypm_lib::values::{err::FypmError, structs::TaskWarriorExported};

pub fn verify_selected_tasks(tasks_json: &Vec<TaskWarriorExported>) -> Result<bool, FypmError> {
    let mut tasks_descriptions = Vec::new();
    for task in tasks_json.clone() {
        tasks_descriptions.push(task.description);
    }

    println!(
        "These are the selected tasks: {}",
        tasks_descriptions.join(" / ")
    );

    let confirmation = Confirm::new()
        .with_prompt("Do you want to continue?")
        .interact()
        .unwrap();

    Ok(confirmation)
}
