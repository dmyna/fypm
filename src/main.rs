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

use utils::enums::Commands;
//#endregion
//#region           Structs && Enums
#[derive(Parser)]
#[command(name = "fypm")]
#[command(version = "0.1.0")]
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
    static ref MAIN_DB_FILE: String = DB_PATH.to_string() + "/fypm.db";
    static ref CONFIG_PATH: String = env::var("FYPM_CONFIG").unwrap_or_else(|_| dirs::home_dir()
        .unwrap()
        .join(".config/fypm")
        .to_string_lossy()
        .into_owned());
}
//#endregion
//#region           Implementation
fn main() {
    handlers::database::DBHandler.ensure_db_path().unwrap();
    handlers::config::ConfigHandler::ensure_config_path().unwrap();
    handlers::config::ConfigHandler::ensure_config_files().unwrap();

    handlers::config::ConfigHandler::handle_config().unwrap();

    let cli = Cli::parse();

    func::matchs::match_subcommand(&cli.command).unwrap();
}
//#endregion
