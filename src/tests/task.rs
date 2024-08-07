#[cfg(test)]
pub mod done {
    use std::process::{Command, Stdio};
    use std::{thread::sleep, time::Duration};

    use crate::subcommands::task;
    use crate::utils::get::get_json_by_filter;
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
            task::task_done(
                &Some("2,3,4".to_string()),
                &None,
                &None,
                &true,
                &false,
                &false,
            )
            .unwrap();

            let tasks = get_json_by_filter("description.is:'Test'", None).unwrap();

            assert_eq!(tasks.len(), 3);
            assert_eq!(
                tasks
                    .iter()
                    .filter(|task| task.status == "completed")
                    .count(),
                3
            );
        }
    }
}
