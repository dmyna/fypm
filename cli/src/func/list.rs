////////////////////////////////////////////////////////////////////////////////
// fypm - The Dark Souls of productivity.
// Copyright (C) 2023-2024 Rikagaku <contact.rikagaku@gmail.com>
// Copyright (C) 2023-2024 Myna <contact@devmyna.xyz>
//
// fypm is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// fypm is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with fypm. If not, see <https://www.gnu.org/licenses/>.
//
////////////////////////////////////////////////////////////////////////////////

use crate::utils::get;
use fypm_lib::values::{err::FypmError, structs::TaskWarriorExported};

/// Print the number of deleted tasks (by status) filtered by the '+DELETED and -COMPLETED and -PENDING' filter.
/// If `no_parents` is `true`, the filter will also include the '-PARENT' modifier, to only show tasks without parents.
pub fn deleted_tasks(no_parents: &bool) -> Result<(), FypmError> {
    let base_filter = "+DELETED and -COMPLETED and -PENDING";

    let final_filter = if *no_parents {
        format!("({} and -PARENT)", base_filter)
    } else {
        format!("({})", base_filter)
    };

    let tasks_json = get::json_by_filter(final_filter.as_str(), None)?;

    fn get_count(tasks_json: &Vec<TaskWarriorExported>, tag: &String) -> usize {
        tasks_json
            .iter()
            .filter(|task| task.tags.as_ref().map_or(false, |tags| tags.contains(tag)))
            .count()
    }

    let archived = get_count(&tasks_json, &"Archived".to_string());
    let failed = get_count(&tasks_json, &"Failed".to_string());
    let abandoned = get_count(&tasks_json, &"Abandoned".to_string());
    let nocontrol = get_count(&tasks_json, &"NoControl".to_string());

    println!("Archived tasks: {}", archived);
    println!("Failed tasks: {}", failed);
    println!("Abandoned tasks: {}", abandoned);
    println!("NoControl tasks: {}", nocontrol);

    Ok(())
}
/// Print the number of pending tasks (by style, type and tags) filtered by the '-DELETED and -COMPLETED' filter.
/// If `no_parents` is `true`, the filter will also include the '-PARENT' modifier, to only show tasks without parents.
///
/// The output will be a table with the following columns:
/// - All pending tasks: The total number of tasks.
/// - Style: The number of tasks with a specific style.
/// - Type: The number of tasks with a specific type.
/// - Tags: The number of tasks with a specific tag.
///
/// The table will also display the total number of tasks without the respective counting.
pub fn pending_tasks(no_parents: &bool) -> Result<(), FypmError> {
    let divisory = "----------------------";
    let base_filter = "-DELETED and -COMPLETED";

    let final_filter = if *no_parents {
        format!("({} and -PARENT)", base_filter)
    } else {
        format!("({})", base_filter)
    };

    let tasks_json = get::json_by_filter(final_filter.as_str(), None)?;

    let all_pending = tasks_json.len();

    let necessity = tasks_json
        .iter()
        .filter(|task| task.style == Some("Necessity".to_string()))
        .count();
    let apollonian = tasks_json
        .iter()
        .filter(|task| task.style == Some("Apollonian".to_string()))
        .count();
    let dionysian = tasks_json
        .iter()
        .filter(|task| task.style == Some("Dionysian".to_string()))
        .count();

    let habit = tasks_json
        .iter()
        .filter(|task| task.r#type == "Habit".to_string())
        .count();
    let eventual = tasks_json
        .iter()
        .filter(|task| task.r#type == "Eventual".to_string())
        .count();
    let objective = tasks_json
        .iter()
        .filter(|task| task.r#type == "Objective".to_string())
        .count();
    let continuous = tasks_json
        .iter()
        .filter(|task| task.r#type == "Continuous".to_string())
        .count();
    let event = tasks_json
        .iter()
        .filter(|task| task.r#type == "Event".to_string())
        .count();
    let check = tasks_json
        .iter()
        .filter(|task| task.r#type == "Check".to_string())
        .count();

    let mother = tasks_json
        .iter()
        .filter(|task| {
            task.tags.is_some() && task.tags.as_ref().unwrap().contains(&"MOTHER".to_string())
        })
        .count();
    let subtask = tasks_json
        .iter()
        .filter(|task| {
            task.tags.is_some() && task.tags.as_ref().unwrap().contains(&"SUBTASK".to_string())
        })
        .count();

    let style_none = tasks_json.iter().filter(|task| task.style == None).count();

    println!("All pending tasks: {}", all_pending);
    println!("{}", divisory);
    println!("Necessity: {} ({})", necessity, all_pending - necessity);
    println!("Apollonian: {} ({})", apollonian, all_pending - apollonian);
    println!("Dionysian: {} ({})", dionysian, all_pending - dionysian);
    println!("{}", divisory);
    println!("Habit: {} ({})", habit, all_pending - habit);
    println!("Eventual: {} ({})", eventual, all_pending - eventual);
    println!("Objective: {} ({})", objective, all_pending - objective);
    println!("Continuous: {} ({})", continuous, all_pending - continuous);
    println!("Event: {} ({})", event, all_pending - event);
    println!("Check: {} ({})", check, all_pending - check);
    println!("Mother: {} ({})", mother, all_pending - mother);
    println!("SubTask: {} ({})", subtask, all_pending - subtask);
    println!("Style None: {}", style_none);
    println!("");
    println!(
        "OBS: The count in parentheses is the total of tasks without the respective counting."
    );

    Ok(())
}
