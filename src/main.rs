//#region           Crates
use clap::Parser;
use lazy_static::lazy_static;
use std::env;
//#endregion
//#region           Modules
mod func;
mod handlers;
mod subcommands;
mod tests;
mod utils;
mod db;

use utils::enums::Commands;
//#endregion
//#region           Structs && Enums
#[derive(Parser)]
#[command(name = "fypm")]
#[command(version = "0.2.0")]
#[command(about = "Four Years Productivity Manager", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

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

    let cli = Cli::parse();

    func::matchs::match_subcommand(&cli.command).unwrap();
}
//#endregion
