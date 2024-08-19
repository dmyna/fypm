use chrono::NaiveDate;

use crate::func;
use crate::handlers;
use crate::values::enums;
use crate::values::err::FypmError;

// pub fn match_exec_command(
//     executed_command: Result<std::process::Output, Error>,
// ) -> Result<(), Error> {
//     match executed_command {
//         Ok(output) => {
//             if output.status.success() {
//                 println!("{}", str::from_utf8(&output.stdout).unwrap());
//             } else {
//                 eprintln!("{}", str::from_utf8(&output.stderr).unwrap());
//             }

//             Ok(())
//         }
//         Err(e) => {
//             eprintln!("Failed to execute command, error: {}", e);

//             Err(e)
//         }
//     }
// }
pub fn match_date_arg(option: &String, option_arg: Option<&String>) -> [NaiveDate; 2] {
    match option.as_str() {
        "-y" | "--year" => func::date::get_year(option_arg),
        "-m" | "--month" => func::date::get_month(option_arg),
        "-w" | "--week" => func::date::get_week(option_arg),
        _ => {
            panic!("You entered an invalid option to date_args!");
        }
    }
}

pub fn match_verify_script(script: &enums::VerifyScripts) -> Result<(), FypmError> {
    match script {
        enums::VerifyScripts::Aliases => handlers::aliases::verify_aliases_tasks(),
    }
}
