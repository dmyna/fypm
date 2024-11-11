#[cfg(test)]
/// Mock for testing. Database cleanup is automated with the `drop` implementation.
/// Ensure that the instance lives until the end of the test suite.
pub struct Mock;

#[cfg(test)]
impl Mock {
    /// Create the CONTROL_TASK `(1)` and 3 equal tests named 'Test' `(2,3,4)`.
    pub fn mock_tasks(&self) {
        use std::process::{Command, Stdio};

        Command::new("task")
            .args(["add", "CONTROL_TASK", "TYPE:Objective"])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .unwrap();

        for _ in 0..3 {
            Command::new("task")
                .args(["add", "Test", "TYPE:Objective"])
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .output()
                .unwrap();
        }
    }
    pub fn mock_db(&self) {
        use crate::handlers::{config, database};

        database::DBHandler::ensure_db_path().unwrap();
        database::DBHandler::ensure_db().unwrap();
        config::ConfigHandler::ensure_config_path().unwrap();
        config::ConfigHandler::ensure_config_files().unwrap();
    }
    pub fn unmock_db(&self) {
        use crate::DB_PATH;
        use std::fs;

        // Clean taskwarrior db
        fs::remove_dir_all("/home/fypm/.task").unwrap();
        // Clean fypm db
        fs::remove_dir_all(DB_PATH.to_string()).unwrap();

        println!("Database cleaned");
    }
}

#[cfg(test)]
impl Drop for Mock {
    fn drop(&mut self) {
        self.unmock_db();
    }
}