#[derive(Clone, serde::Deserialize, Debug)]
#[allow(unused)]
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
    pub tags: Option<Vec<String>>,
    pub urgency: f64,
}
#[derive(serde::Deserialize, Debug)]
#[allow(unused)]
pub struct TimeWarriorExported {
    pub id: i32,
    pub start: String,
    pub end: Option<String>,
    pub tags: Option<Vec<String>>,
}
pub struct GetJsonByFilterOptions {
    pub quantity: Option<usize>,
}