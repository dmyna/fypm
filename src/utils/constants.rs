use super::structs::GetJsonByFilterOptions;

pub const LAST_TASK_PATH: &str = "/tmp/.last_task";

pub const DEFAULT_GET_JSON_OPTIONS: Option<GetJsonByFilterOptions> =
    Some(GetJsonByFilterOptions { quantity: Some(1) });