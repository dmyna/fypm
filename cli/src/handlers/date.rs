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

use chrono::{Duration, NaiveDate};

pub struct NaiveDateIter {
    start: NaiveDate,
    end: NaiveDate,
}
impl NaiveDateIter {
    pub fn new(start: NaiveDate, end: NaiveDate) -> NaiveDateIter {
        NaiveDateIter { start, end }
    }
}
impl Iterator for NaiveDateIter {
    type Item = NaiveDate;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            let next = self.start;
            self.start = self.start.checked_add_signed(Duration::days(1)).unwrap();
            Some(next)
        } else {
            None
        }
    }
}
