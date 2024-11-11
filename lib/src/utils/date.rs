use chrono::{Datelike, Duration, Local, NaiveDate, Weekday};

const INVALID_DATE_MSG: &str = "You entered a invalid date!";

pub fn get_year(year: Option<&String>) -> [NaiveDate; 2] {
    let cur_year = Local::now().year();
    let initial_date: NaiveDate;
    let final_date: NaiveDate;
    let final_year: i32;

    if let Some(year) = year {
        let parsed_year = year.trim().parse::<i32>().unwrap();

        final_year = parsed_year;
    } else {
        final_year = cur_year;
    }

    initial_date = NaiveDate::from_ymd_opt(final_year, 1, 1).unwrap();
    final_date = NaiveDate::from_ymd_opt(final_year + 1, 1, 1).unwrap();

    [initial_date, final_date]
}
pub fn get_month(month: Option<&String>) -> [NaiveDate; 2] {
    let cur_year = Local::now().year();
    let cur_mon = Local::now().month();
    let initial_date: NaiveDate;
    let final_date: NaiveDate;
    let final_month: u32;

    if let Some(month) = month {
        let parsed_month = month.trim().parse::<u32>().unwrap();
        if parsed_month > 12 {
            panic!("{INVALID_DATE_MSG}");
        }

        final_month = parsed_month;
    } else {
        final_month = cur_mon;
    }

    initial_date = NaiveDate::from_ymd_opt(cur_year, final_month, 1).unwrap();

    if final_month == 12 {
        final_date = NaiveDate::from_ymd_opt(cur_year + 1, 1, 1).unwrap();
    } else {
        final_date = NaiveDate::from_ymd_opt(cur_year, final_month + 1, 1).unwrap();
    }

    [initial_date, final_date]
}

pub fn get_week(week: Option<&String>) -> [NaiveDate; 2] {
    let cur_year = Local::now().year();
    let initial_date: NaiveDate;
    let final_date: NaiveDate;
    let final_week: u32;

    if let Some(week) = week {
        let parsed_week = week.trim().parse::<u32>().unwrap();
        if parsed_week > 53 {
            panic!("{INVALID_DATE_MSG}");
        }

        final_week = parsed_week;
    } else {
        final_week = Local::now().iso_week().week();
    }

    initial_date =
        NaiveDate::from_isoywd_opt(cur_year, final_week, Weekday::Mon).expect("Week not found!");

    let get_final_date = NaiveDate::from_isoywd_opt(cur_year, final_week + 1, Weekday::Mon);
    if let Some(received_final_date) = get_final_date {
        final_date = received_final_date;
    } else {
        final_date = NaiveDate::from_isoywd_opt(cur_year + 1, 1, Weekday::Mon).unwrap();
    }

    [initial_date, final_date]
}
pub fn match_aliases(date: &String) -> String {
    match date.as_str() {
        "today" => Local::now().format("%Y-%m-%d").to_string(),
        "yesterday" => (Local::now() - Duration::days(1))
            .format("%Y-%m-%d")
            .to_string(),
        "tomorrow" => (Local::now() + Duration::days(1))
            .format("%Y-%m-%d")
            .to_string(),
        _ => date.to_string(),
    }
}
