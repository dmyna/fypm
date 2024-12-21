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
    /// Ensures that the database directory exists by creating it if it does not.
    ///
    /// Returns:
    ///     Result<(), Error>: An Ok result if the directory is successfully created
    ///     or already exists, otherwise returns an error.
    pub fn ensure_db_path() -> Result<(), Error> {
        fs::create_dir_all(DB_PATH.to_string())?;

        Ok(())
    }
    /// Ensures that the database is properly initialized by running all
    /// of the SQL migrations. This function should be called before any
    /// other database operations.
    ///
    /// Returns:
    ///     Result<(), Error>: An Ok result if the database is successfully ensured,
    ///     otherwise returns an error.
    pub fn ensure_db() -> Result<(), Error> {
        let mut conn = SqliteConnection::establish(DATABASE_URL.to_string().as_str()).unwrap();

        conn.run_pending_migrations(MIGRATIONS).unwrap();

        Ok(())
    }
}
