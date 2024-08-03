use super::structs::GetJsonByFilterOptions;

pub const LAST_TASK_PATH: &str = "/tmp/.last_task";

//. DEV: Switch this string for a dynamic system
pub const CONTROL_TASK: &str = "5c847c7e-c7eb-44f6-ad7e-29cc989c8854";

pub const DEFAULT_GET_JSON_OPTIONS: Option<GetJsonByFilterOptions> =
    Some(GetJsonByFilterOptions { quantity: Some(1) });

pub const DEFAULT_CONFIG_FILES: [&str; 6] = [
    "task.fypm.ini",
    "uda.fypm.ini",
    "report.fypm.ini",
    "urgency.fypm.ini",
    "colors.fypm.ini",
    "overlay.fypm.ini",
];