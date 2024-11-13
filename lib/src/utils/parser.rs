use regex::Regex;
use std::str::FromStr;
use chrono::{DateTime, Local, Offset, ParseError, NaiveDate};

use super::matchs;

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