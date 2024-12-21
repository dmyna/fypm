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

use dialoguer::Confirm;

use fypm_lib::values::{err::FypmError, structs::TaskWarriorExported};

/// Verify the selected tasks with the user.
///
/// This function asks the user to confirm if the selected tasks are correct.
/// If the user confirms, it returns Ok(true), otherwise it returns Err(FypmError).
///
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
