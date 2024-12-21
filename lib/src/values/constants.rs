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

use super::structs::GetJsonByFilterOptions;

pub const LAST_TASK_PATH: &str = "/tmp/.last_task";

//. DEV: Switch this string for a dynamic system
pub const CONTROL_TASK: &str = "5c847c7e-c7eb-44f6-ad7e-29cc989c8854";

pub const DEFAULT_GET_JSON_OPTIONS: Option<GetJsonByFilterOptions> = Some(GetJsonByFilterOptions {
    quantity: Some(1),
    aditional_overrides: None,
});

pub const DEFAULT_CONFIG_FILES: [&str; 6] = [
    "task.fypm.ini",
    "uda.fypm.ini",
    "report.fypm.ini",
    "urgency.fypm.ini",
    "colors.fypm.ini",
    "overlay.fypm.ini",
];
