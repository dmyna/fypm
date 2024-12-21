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

use chrono::{DateTime, Local, NaiveDate, Offset, ParseError};
use regex::Regex;
use std::str::FromStr;

use super::matchs;

/// Transform a date string received from taskwarrior from the format
/// "YYYYMMDDTHHMMSSZ" to "YYYY-MM-DDTHH:MM:SSZ" and then to the local
/// timezone. The space in the middle of the date string is added because
/// otherwise the date would be parsed as "YYYY-MM-DD:00:00:00", which is
/// not what we want.
///
/// # Errors
///
/// This function returns a `ParseError` if the date string can't be parsed
/// or if the date string is not in the correct format.
pub fn transform_dates_to_iso(received_time: String) -> Result<String, ParseError> {
    let transformed_time_str = Regex::new(r"(\d{4})(\d{2})(\d{2})T(\d{2})(\d{2})(\d{2})Z")
        .unwrap()
        .replace(&received_time, "$1-$2-$3 T $4:$5:$6 Z")
        .replace(" ", "")
        .to_string();
    // I put this space because dates the day case wasn't showing.
    // Dates like "20240101T000000Z" were showing as "2024-01-00:00:00".

    let parsed_time_str = DateTime::parse_from_rfc3339(&transformed_time_str.as_str())?;

    let local_offset = Local::now().offset().fix();

    let final_time = parsed_time_str
        .with_timezone(&local_offset)
        .format("%Y-%m-%dT%H:%M:%S")
        .to_string();

    Ok(final_time)
}
/// Given a vector of strings representing date arguments, parse the strings into
/// `NaiveDate` objects and return them as an array of two elements. If the vector
/// has three elements, the first element is the initial date and the third element
/// is the final date. If the vector has two elements, it is processed by
/// `matchs::match_date_arg` and the result is returned.
///
/// # Errors
///
/// This function will panic if the vector of date arguments has more than three
/// elements. It will also panic if the date strings are not in the correct format.
pub fn date_period(date_args: &Vec<String>) -> [NaiveDate; 2] {
    let args_len = date_args.len();
    if args_len > 3 {
        panic!("You entered too many arguments to date_args!");
    }

    let initial_date: NaiveDate;
    let final_date: NaiveDate;

    if args_len == 3 {
        let initial_date_str = date_args.get(0).unwrap();
        let final_date_str = date_args.get(2).unwrap();

        initial_date = NaiveDate::from_str(&initial_date_str).unwrap();
        final_date = NaiveDate::from_str(&final_date_str).unwrap();
    } else {
        let option: &String = date_args.get(0).unwrap();

        let mut option_arg: Option<&String> = None;

        if args_len == 2 {
            option_arg = Some(date_args.get(1).unwrap());
        }

        [initial_date, final_date] = matchs::match_date_arg(option, option_arg);
    }

    [initial_date, final_date]
}
