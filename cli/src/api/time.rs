use chrono::NaiveDate;
use std::str::FromStr;

use crate::utils::get;
use rocket::form::FromForm;

use fypm_lib::utils::date;

#[derive(FromForm)]
pub struct TimeListingQuery {
    pub start_date: String,
    pub end_date: String,
}


/// Return a json array of timew entries between given start and end date.
///
/// # Path
///
/// /time/[filter]?start_date=<start_date>&end_date=<end_date>
///
/// # Query Parameters
///
/// * `start_date` - The start date of the range to query. If not given today's date is used.
/// * `end_date` - The end date of the range to query. If not given, the day after the start date is used.
///
/// # Filter
///
/// If `filter` is given, it will be passed to `timew export` as a filter.
///
/// # Return
///
/// * A json array of `TimeWarriorExported` structs, or an error message if there is an error while
///   running `timew export`.
#[get("/time/log?<params..>")]
pub fn listing(params: TimeListingQuery) -> String {
    let start= NaiveDate::from_str(&date::match_aliases(&params.start_date)).unwrap();
    let end = NaiveDate::from_str(&date::match_aliases(&params.end_date)).unwrap();

    let data = get::timew_entries(start, end).unwrap();

    serde_json::to_string(&data).expect("Failed to serialize json!")
}
