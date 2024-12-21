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

use colored::*;

/// Prints a full divisory line in the terminal, given the
/// terminal size if it could be determined, or a default
/// of 30 characters if it could not.
pub fn print_full_divisory() -> () {
    let divisory_char = '─';

    if let Some((terminal_size::Width(width), _)) = terminal_size::terminal_size() {
        for _ in 0..width {
            print!("{}", divisory_char.to_string().bright_black());
        }
    } else {
        for _ in 0..30 {
            print!("{}", divisory_char.to_string().bright_black());
        }
    }

    println!();
}
