//#region           Crates
use std::fs;
use std::io::Error;

use diesel::Connection;
use diesel::SqliteConnection;
use diesel_migrations::MigrationHarness;

use crate::db::models::MIGRATIONS;
use crate::{DATABASE_URL, DB_PATH};

//#endregion
//#region           Modules
//#endregion
//#region           Structs
#[derive(Debug)]
pub struct DBHandler;
//#endregion
//#region           Implementation
impl DBHandler {
    pub fn ensure_db_path() -> Result<(), Error> {
        fs::create_dir_all(DB_PATH.to_string())?;

        Ok(())
    }

    pub fn ensure_db() -> Result<(), Error> {
        let mut conn = SqliteConnection::establish(DATABASE_URL.to_string().as_str()).unwrap();

        conn.run_pending_migrations(MIGRATIONS).unwrap();

        Ok(())
    }
}
