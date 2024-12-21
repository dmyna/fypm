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
pub fn time_log(params: TimeListingQuery) -> String {
    let start = NaiveDate::from_str(&date::match_aliases(&params.start_date)).unwrap();
    let end = NaiveDate::from_str(&date::match_aliases(&params.end_date)).unwrap();

    let data = get::timew_entries(start, end).unwrap();

    serde_json::to_string(&data).expect("Failed to serialize json!")
}
