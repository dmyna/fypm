use chrono::NaiveDate;

use super::date;

pub fn match_date_arg(option: &String, option_arg: Option<&String>) -> [NaiveDate; 2] {
    match option.as_str() {
        "-y" | "--year" => date::get_year(option_arg),
        "-m" | "--month" => date::get_month(option_arg),
        "-w" | "--week" => date::get_week(option_arg),
        _ => {
            panic!("You entered an invalid option to date_args!");
        }
    }
}