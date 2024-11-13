use chrono::NaiveDate;
use std::str::FromStr;

use crate::utils::get;

use fypm_lib::utils::date;

/// Get a json containing all timew entries between <start_date> and <end_date>
///
/// The date format can be any of the following:
///
/// * `YYYY-MM-DD`
/// * `yesterday`
/// * `today`
/// * `tomorrow`
/// * `this week` (or `this mon`, `this tue`, etc.)
/// * `last week` (or `last mon`, `last tue`, etc.)
/// * `next week` (or `next mon`, `next tue`, etc.)
/// * `next month`
/// * `last month`
/// * `next year`
/// * `last year`
///
/// The date is localized according to the user's locale.
#[get("/time/<start_date>/<end_date>")]
pub fn listing(start_date: &str, end_date: &str) -> String {
    let start= NaiveDate::from_str(&date::match_aliases(&start_date.to_string())).unwrap();
    let end = NaiveDate::from_str(&date::match_aliases(&end_date.to_string())).unwrap();

    let data = get::timew_entries(start, end).unwrap();

    serde_json::to_string(&data).expect("Failed to serialize json!")
}
