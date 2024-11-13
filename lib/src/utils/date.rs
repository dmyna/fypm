use chrono::{Datelike, Duration, Local, NaiveDate, Weekday};

const INVALID_DATE_MSG: &str = "You entered a invalid date!";

/// Returns an array containing the start and end dates for the specified year.
///
/// If a year is provided, it parses the year and returns the dates for January 1st and January 1st of the following year.
/// If no year is provided, it uses the current year based on the local time.
///
/// # Arguments
///
/// * `year` - An optional reference to a string representing the year.
///
/// # Returns
///
/// An array of two `NaiveDate` objects: the first represents January 1st of the specified or current year,
/// and the second represents January 1st of the following year.
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
/// Returns an array containing the start and end dates for the specified month.
///
/// If a month is provided, it parses the month and returns the dates for the first day of the specified month
/// and the first day of the next month.
/// If no month is provided, it uses the current month based on the local time.
///
/// # Arguments
///
/// * `month` - An optional reference to a string representing the month.
///
/// # Returns
///
/// An array of two `NaiveDate` objects: the first represents the first day of the specified or current month,
/// and the second represents the first day of the next month.
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

/// Returns an array containing the start and end dates for the specified week.
///
/// If a week is provided, it parses the week and returns the dates for the first day of the specified week
/// and the first day of the next week.
/// If no week is provided, it uses the current week based on the local time.
///
/// # Arguments
///
/// * `week` - An optional reference to a string representing the week.
///
/// # Returns
///
/// An array of two `NaiveDate` objects: the first represents the first day of the specified or current week,
/// and the second represents the first day of the next week.
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
/// Matches the date string to the corresponding date, given by the following possibilities:
///
/// * "today": The current date.
/// * "yesterday": The day before today.
/// * "tomorrow": The day after today.
///
/// If the date string does not match any of the above possibilities, it is returned as is.
///
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
