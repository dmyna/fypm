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

#[cfg(test)]
pub mod done {
    use std::process::{Command, Stdio};
    use std::{thread::sleep, time::Duration};

    use crate::commands::task;
    use crate::utils::get::json_by_filter;
    use fypm_lib::values::structs::TaskWarriorStatus;
    use super::super::lib::mock::Mock;

    #[test]
    pub fn simple_done() {
        // Prepair test
        let mock = Mock;
        {
            mock.mock_db();
            mock.mock_tasks();

            Command::new("task")
                .args(["1", "start"])
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .output()
                .unwrap();

            sleep(Duration::from_millis(1000));
        }

        // Test
        {
            task::update::done(
                &Some("2,3,4".to_string()),
                &None,
                &None,
                &true,
                &false,
                &false,
            )
            .unwrap();

            let tasks = json_by_filter("description.is:'Test'", None).unwrap();

            assert_eq!(tasks.len(), 3);
            assert_eq!(
                tasks
                    .iter()
                    .filter(|task| task.status == TaskWarriorStatus::Completed)
                    .count(),
                3
            );
        }
    }
}
