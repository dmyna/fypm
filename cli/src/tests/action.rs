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

#[cfg(test)]
use fypm_lib::values;
#[cfg(test)]
use crate::func;
#[cfg(test)]
use fypm_lib::values::structs::TaskWarriorStatus;

#[test]
fn verify_if_wt_is_allday() {
    let json = values::structs::TaskWarriorExported {
        id: 1,
        description: "test".to_string(),
        tags: Some(vec!["Test_Tag".to_string()]),
        project: None,
        state: "Time".to_string(),
        r#type: "Task".to_string(),
        wt: "AllDay".to_string(),
        inforelat: None,
        seq_current: None,
        seq_next: None,
        seq_prev: None,
        alias: None,
        style: None,
        entry: "2023-08-22T00:00:00Z".to_string(),
        modified: "2023-08-22T00:00:00Z".to_string(),
        due: None,
        parent: None,
        status: TaskWarriorStatus::Pending,
        uuid: "1".to_string(),
        annotations: None,
        urgency: 0.0,
        effort: None,
        quadrant: None
    };

    let exec = func::action::verify_if_wt_is_allday(&json);

    assert!(exec.is_err());
}
