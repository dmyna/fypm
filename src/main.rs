//#region           Crates
use lazy_static::lazy_static;
use clap::Parser;
use std::env;
//#endregion
//#region           Modules
mod func;
mod handlers;
mod commands;
mod tests;
mod utils;
mod db;
mod values;
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

    let cli = values::enums::Cli::parse();

    commands::matching(&cli.commands).unwrap();
}
//#endregion
