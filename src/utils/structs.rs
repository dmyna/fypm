#[derive(serde::Deserialize)]
#[allow(unused)]
pub struct TaskWarriorExported {
    pub id: u32,
    #[serde(rename = "STATE")]
    pub state: String,
    #[serde(rename = "STYLE")]
    pub style: Option<String>,
    #[serde(rename = "TYPE")]
    pub r#type: Option<String>,
    #[serde(rename = "WT")]
    pub wt: String,
    pub description: String,
    pub entry: String,
    pub modified: String,
    pub project: Option<String>,
    pub status: String,
    pub uuid: String,
    pub tags: Option<Vec<String>>,
    pub urgency: f64,
}
#[derive(serde::Deserialize)]
#[allow(unused)]
pub struct TimeWarriorExported {
    pub id: i32,
    pub start: String,
    pub end: Option<String>,
    pub tags: Option<Vec<String>>,
}
