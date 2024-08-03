#[cfg(test)]
use crate::subcommands::task;

#[cfg(test)]
fn mock_tasks() {
    use std::process::Command;

    Command::new("task")
        .args(["add", "Test1"])
        .output()
        .unwrap();

    Command::new("task")
        .args(["add", "Test2"])
        .output()
        .unwrap();

    Command::new("task")
        .args(["add", "Test3"])
        .output()
        .unwrap();
}

#[test]
pub fn test_done() {
    mock_tasks();

    let execute = task::task_done(&Some("1,2,3".to_string()), &None);

    assert!(execute.is_ok());
}
