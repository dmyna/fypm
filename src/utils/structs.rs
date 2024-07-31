#[derive(Clone, serde::Deserialize, Debug)]
pub struct TaskAnnotation {
    pub entry: String,
    pub description: String,
}
#[derive(Clone, serde::Deserialize, Debug)]
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
    pub entry: String,
    pub modified: String,
    pub project: Option<String>,
    pub status: String,
    pub uuid: String,
    pub annotations: Option<Vec<TaskAnnotation>>,
    pub tags: Option<Vec<String>>,
    pub urgency: f64,
}
#[derive(serde::Deserialize, Debug)]
pub struct TimeWarriorExported {
    pub id: i32,
    pub start: String,
    pub end: Option<String>,
    pub tags: Option<Vec<String>>,
}

pub struct GetJsonByFilterOptions {
    pub quantity: Option<usize>,
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


pub enum TaskWarriorUserScopeProperty {
    Tag,
}
pub enum TaskWarriorUrgencyConfigScope {
    UDA,
    Common,
    User {
        property: TaskWarriorUserScopeProperty,
    },
}