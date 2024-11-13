use chrono::NaiveDate;

use super::date;

/// Matches the date argument to a NaiveDate range.
///
/// # Options
///
/// The available options for the date argument are:
///
/// * `-y` or `--year`: To get a year range.
/// * `-m` or `--month`: To get a month range.
/// * `-w` or `--week`: To get a week range.
///
/// # Panics
///
/// Panics if an invalid option is provided.
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
