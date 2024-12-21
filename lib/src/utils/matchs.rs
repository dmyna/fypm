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
