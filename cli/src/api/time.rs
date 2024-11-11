use chrono::NaiveDate;
use std::str::FromStr;

use crate::utils::get;

use fypm_lib::utils::date;

#[get("/time/<start_date>/<end_date>")]
pub fn listing(start_date: &str, end_date: &str) -> String {
    let start= NaiveDate::from_str(&date::match_aliases(&start_date.to_string())).unwrap();
    let end = NaiveDate::from_str(&date::match_aliases(&end_date.to_string())).unwrap();

    let data = get::timew_entries(start, end).unwrap();

    serde_json::to_string(&data).expect("Failed to serialize json!")
}
