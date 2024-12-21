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
use clap::Parser;
use lazy_static::lazy_static;
use std::env;
//#endregion
//#region           Modules
mod commands;
mod db;
mod func;
mod handlers;
mod tests;
mod utils;
//#endregion
//#region           Constants
lazy_static! {
    static ref DB_PATH: String = env::var("FYPM_DB").unwrap_or_else(|_| dirs::home_dir()
        .unwrap()
        .join(".local/share/fypm")
        .to_string_lossy()
        .into_owned());
    static ref CONFIG_PATH: String = env::var("FYPM_CONFIG").unwrap_or_else(|_| dirs::home_dir()
        .unwrap()
        .join(".config/fypm")
        .to_string_lossy()
        .into_owned());

    #[derive(Debug)]
    static ref DATABASE_URL: String = DB_PATH.to_string() + "/fypm.db";
}
//#endregion
//#region           Implementation
fn main() {
    handlers::database::DBHandler::ensure_db_path().unwrap();
    handlers::database::DBHandler::ensure_db().unwrap();
    handlers::filters::FiltersHandler::ensure_defaults().unwrap();

    handlers::config::ConfigHandler::ensure_config_path().unwrap();
    handlers::config::ConfigHandler::ensure_config_files().unwrap();

    handlers::config::ConfigHandler::handle_config().unwrap();

    let cli = commands::Cli::parse();

    commands::matching(&cli.commands).unwrap();

    ()
}
//#endregion
