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

use std::collections::BTreeMap;

#[derive(Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize, Debug)]
pub enum TaskWarriorStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "deleted")]
    Deleted,
    #[serde(rename = "recurring")]
    Recurring,
    #[serde(rename = "waiting")]
    Waiting,
}

#[derive(Clone, serde::Deserialize, serde::Serialize, Debug)]
pub struct TaskAnnotation {
    pub entry: String,
    pub description: String,
}
#[derive(Clone, serde::Deserialize, serde::Serialize, Debug)]
pub struct TaskWarriorExported {
    pub id: u32,
    #[serde(rename = "STATE")]
    pub state: String,
    #[serde(rename = "STYLE")]
    pub style: Option<String>,
    #[serde(rename = "TYPE")]
    pub r#type: String,
    #[serde(rename = "WT")]
    pub wt: String,
    #[serde(rename = "INFORELAT")]
    pub inforelat: Option<String>,
    #[serde(rename = "ALIAS")]
    pub alias: Option<String>,
    #[serde(rename = "SEQ_CURRENT")]
    pub seq_current: Option<String>,
    #[serde(rename = "SEQ_NEXT")]
    pub seq_next: Option<String>,
    #[serde(rename = "SEQ_PREVIOUS")]
    pub seq_prev: Option<String>,
    pub description: String,
    pub due: Option<String>,
    pub entry: String,
    pub modified: String,
    pub parent: Option<String>,
    pub project: Option<String>,
    pub status: TaskWarriorStatus,
    pub uuid: String,
    pub annotations: Option<Vec<TaskAnnotation>>,
    pub tags: Option<Vec<String>>,
    pub urgency: f64,
    pub effort: Option<String>,
    pub quadrant: Option<String>,
}
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone )]
pub struct TimeWarriorExported {
    pub id: i32,
    pub start: String,
    pub end: Option<String>,
    pub tags: Option<Vec<String>>,
}

pub struct GetJsonByFilterOptions {
    pub quantity: Option<usize>,
    pub aditional_overrides: Option<Vec<String>>,
}

pub struct TaskWarriorReportConfig {
    pub columns: Option<Vec<String>>,
    pub labels: Option<Vec<String>>,
    pub sort: Option<Vec<String>>,
    pub filter: Option<String>,
}
pub struct TaskWarriorUDAConfig {
    pub r#type: String,
    pub label: String,
    pub values: Option<Vec<String>>,
    pub default: Option<String>,
}
pub struct TaskWarriorUrgencyConfig {
    pub coefficient: f32,
    pub scope: TaskWarriorUrgencyConfigScope,
}

impl Default for TaskWarriorReportConfig {
    fn default() -> Self {
        Self {
            columns: None,
            labels: None,
            sort: None,
            filter: None,
        }
    }
}
impl Default for TaskWarriorUDAConfig {
    fn default() -> Self {
        Self {
            r#type: String::new(),
            label: String::new(),
            values: None,
            default: None,
        }
    }
}
impl Default for TaskWarriorUrgencyConfig {
    fn default() -> Self {
        Self {
            coefficient: 0.0,
            scope: TaskWarriorUrgencyConfigScope::Common,
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum TaskWarriorUserScopeProperty {
    Tag,
}

#[derive(PartialEq, Eq)]
pub enum TaskWarriorUrgencyConfigScope {
    UDA,
    Common,
    User {
        property: TaskWarriorUserScopeProperty,
    },
}
#[derive(Clone)]
pub struct FypmConfigFile {
    pub name: String,
    pub map: BTreeMap<String, String>,
}