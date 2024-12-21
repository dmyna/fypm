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

use crate::utils::get;

#[derive(FromForm)]
pub struct TaskQuery {
    pub filter: Option<String>,
}

#[get("/task?<params..>")]
pub fn task(params: TaskQuery) -> String {
    let filter = if let Some(filter) = params.filter {
        filter
    } else {
        "".to_string()
    };

    let data = get::json_by_filter(&filter, None).unwrap();

    serde_json::to_string(&data).expect("Failed to serialize json!")
}
