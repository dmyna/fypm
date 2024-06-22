use crate::func::matchs;

use chrono::NaiveDate;
use std::str::FromStr;

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