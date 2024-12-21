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

use std::io::{Error, ErrorKind};

/// Verifies if a given string is a valid HEX color.
///
/// # Args
///
/// * `string` - A `String` containing the HEX color to be verified.
///
/// # Returns
///
/// * `Result<bool, std::io::Error>` - `Ok(true)` if the given string is a valid HEX color,
///   `Err(std::io::Error)` if the given string is not a valid HEX color.
///
/// # Errors
///
/// * `std::io::ErrorKind::InvalidInput` - If the given string is not a valid HEX color.
pub fn verify_hex(string: String) -> Result<bool, Error> {
    let raw = string.strip_prefix("#").unwrap_or(string.as_str());

    if raw.len() == 3 || raw.len() == 6 {
        let parse_result = u32::from_str_radix(raw, 16);

        match parse_result {
            Ok(_) => Ok(true),
            Err(_) => Err(Error::new(
                ErrorKind::InvalidInput,
                "Invalid HEX!".to_string(),
            )),
        }
    } else {
        Err(Error::new(
            ErrorKind::InvalidInput,
            "The hex must be 3 or 6 characters long!".to_string(),
        ))
    }
}
