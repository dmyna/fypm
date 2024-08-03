use crate::utils::comments::{COLORS_CONFIG_COMMENT, OVERLAY_CONFIG_COMMENT, REPORT_CONFIG_COMMENT, TASK_CONFIG_COMMENT, UDA_CONFIG_COMMENT, URGENCY_CONFIG_COMMENT};
use crate::utils::constants::DEFAULT_CONFIG_FILES;
use crate::utils::enums::{FypmReports, FypmUDAs, FypmUrgency};
use crate::utils::err::{FypmError, FypmErrorKind};
use crate::utils::structs::{
    FypmConfigFile, TaskWarriorReportConfig, TaskWarriorUDAConfig, TaskWarriorUrgencyConfig,
    TaskWarriorUrgencyConfigScope, TaskWarriorUserScopeProperty,
};
use crate::CONFIG_PATH;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::io::Write;

pub struct FypmConfigs {
    pub uda: BTreeMap<FypmUDAs, TaskWarriorUDAConfig>,
    pub report: BTreeMap<FypmReports, TaskWarriorReportConfig>,
    pub urgency: BTreeMap<FypmUrgency, TaskWarriorUrgencyConfig>,
}

pub struct ConfigHandler;
impl ConfigHandler {
    pub fn ensure_config_path() -> Result<(), FypmError> {
        fs::create_dir_all(CONFIG_PATH.to_string()).unwrap();

        Ok(())
    }
    pub fn ensure_config_files() -> Result<(), FypmError> {
        fn ensure_file_existence(config_file: &str, initial_content: &str) {
            let file_path = Path::new(&CONFIG_PATH.to_string()).join(config_file);

            if !file_path.exists() {
                let mut file = fs::File::create(file_path).unwrap();

                file.write_all(initial_content.as_bytes()).unwrap();
            }
        }

        ensure_file_existence(DEFAULT_CONFIG_FILES[0], TASK_CONFIG_COMMENT);
        ensure_file_existence(DEFAULT_CONFIG_FILES[1], UDA_CONFIG_COMMENT);
        ensure_file_existence(DEFAULT_CONFIG_FILES[2], REPORT_CONFIG_COMMENT);
        ensure_file_existence(DEFAULT_CONFIG_FILES[3], URGENCY_CONFIG_COMMENT);
        ensure_file_existence(DEFAULT_CONFIG_FILES[4], COLORS_CONFIG_COMMENT);
        ensure_file_existence(DEFAULT_CONFIG_FILES[5], OVERLAY_CONFIG_COMMENT);

        Ok(())
    }
    pub fn create_config_defaults() -> FypmConfigs {
        FypmConfigs {
            report: BTreeMap::from([
                (FypmReports::Waiting, TaskWarriorReportConfig {
                    sort: Some(vec!["wait+".to_string()]),
                    ..Default::default()
                }), (FypmReports::Next, TaskWarriorReportConfig {
                    columns: Some(vec![
                        "entry.age".to_string(),
                        "STYLE".to_string(),
                        "TYPE".to_string(),
                        "WT".to_string(),
                        "estimate".to_string(),
                        "due.relative".to_string(),
                        "project".to_string(),
                        "id".to_string(),
                        "description".to_string(),
                        "urgency".to_string(),
                    ]),
                    labels: Some(vec![
                        "Age".to_string(),
                        "Style".to_string(),
                        "Type".to_string(),
                        "WorkTime".to_string(),
                        "Est".to_string(),
                        "Due".to_string(),
                        "Project".to_string(),
                        "ID".to_string(),
                        "Description".to_string(),
                        "Urg".to_string(),
                    ]),
                    sort: Some(vec!["urgency-".to_string(), "ALARM-".to_string()]),
                    ..Default::default()
                }),
                (FypmReports::List, TaskWarriorReportConfig {
                    columns: Some(vec![
                        "entry.age".to_string(),
                        "STYLE".to_string(),
                        "TYPE".to_string(),
                        "project".to_string(),
                        "tags".to_string(),
                        "recur".to_string(),
                        "WT".to_string(),
                        "GOAL".to_string(),
                        "due.relative".to_string(),
                        "id".to_string(),
                        "description".to_string(),
                        "urgency".to_string(),
                    ]),
                    labels: Some(vec![
                        "Age".to_string(),
                        "Style".to_string(),
                        "Type".to_string(),
                        "Project".to_string(),
                        "Tags".to_string(),
                        "Recur".to_string(),
                        "WorkTime".to_string(),
                        "Goal".to_string(),
                        "Due".to_string(),
                        "ID".to_string(),
                        "Description".to_string(),
                        "Urg".to_string(),
                    ]),
                    sort: Some(vec!["urgency+".to_string()]),
                    filter: Some("status:pending -WAITING -Ghost".to_string()),
                    ..Default::default()
                }),
                (FypmReports::All, TaskWarriorReportConfig {
                    columns: Some(vec![
                        "uuid.short".to_string(),
                        "entry.age".to_string(),
                        "STYLE".to_string(),
                        "TYPE".to_string(),
                        "project".to_string(),
                        "tags".to_string(),
                        "recur".to_string(),
                        "WT".to_string(),
                        "GOAL".to_string(),
                        "scheduled.countdown".to_string(),
                        "due.relative".to_string(),
                        "id".to_string(),
                        "description".to_string(),
                        "urgency".to_string(),
                    ]),
                    labels: Some(vec![
                        "UUID".to_string(),
                        "Age".to_string(),
                        "Style".to_string(),
                        "Type".to_string(),
                        "Project".to_string(),
                        "Tags".to_string(),
                        "Recur".to_string(),
                        "WorkTime".to_string(),
                        "Goal".to_string(),
                        "Sched".to_string(),
                        "Due".to_string(),
                        "ID".to_string(),
                        "Description".to_string(),
                        "Urg".to_string(),
                    ]),
                    sort: Some(vec!["urgency+".to_string()]),
                    ..Default::default()
                }),
                (FypmReports::Goals, TaskWarriorReportConfig {
                    columns: Some(vec![
                        "entry.age".to_string(),
                        "TYPE".to_string(),
                        "project".to_string(),
                        "WT".to_string(),
                        "GOAL".to_string(),
                        "due.relative".to_string(),
                        "id".to_string(),
                        "description".to_string(),
                        "urgency".to_string(),
                    ]),
                    labels: Some(vec![
                        "Age".to_string(),
                        "Type".to_string(),
                        "Project".to_string(),
                        "WorkTime".to_string(),
                        "Goal".to_string(),
                        "Due".to_string(),
                        "ID".to_string(),
                        "Description".to_string(),
                        "Urg".to_string(),
                    ]),
                    sort: Some(vec!["GOAL+".to_string()]),
                    filter: Some("status:pending and GOAL.any:".to_string()),
                    ..Default::default()
                }),
                (FypmReports::Alarms, TaskWarriorReportConfig {
                    columns: Some(vec![
                        "entry.age".to_string(),
                        "STYLE".to_string(),
                        "TYPE".to_string(),
                        "project".to_string(),
                        "tags".to_string(),
                        "WT".to_string(),
                        "GOAL".to_string(),
                        "ALARM.relative".to_string(),
                        "due.relative".to_string(),
                        "id".to_string(),
                        "description".to_string(),
                        "urgency".to_string(),
                    ]),
                    labels: Some(vec![
                        "Age".to_string(),
                        "Style".to_string(),
                        "Type".to_string(),
                        "Project".to_string(),
                        "Tags".to_string(),
                        "WorkTime".to_string(),
                        "Goal".to_string(),
                        "Alarm".to_string(),
                        "Due".to_string(),
                        "ID".to_string(),
                        "Description".to_string(),
                        "Urg".to_string(),
                    ]),
                    sort: Some(vec!["ALARM+".to_string()]),
                    filter: Some("status:pending and ALARM.any:".to_string()),
                    ..Default::default()
                }),
                (FypmReports::Const, TaskWarriorReportConfig {
                    columns: Some(vec![
                        "id".to_string(),
                        "entry.age".to_string(),
                        "STYLE".to_string(),
                        "TYPE".to_string(),
                        "project".to_string(),
                        "tags".to_string(),
                        "WT".to_string(),
                        "GOAL".to_string(),
                        "due.relative".to_string(),
                        "description".to_string(),
                        "urgency".to_string(),
                    ]),
                    labels: Some(vec![
                        "ID".to_string(),
                        "Age".to_string(),
                        "Style".to_string(),
                        "Type".to_string(),
                        "Project".to_string(),
                        "TaskWarriorUserScopeProperty::Tag".to_string(),
                        "WorkTime".to_string(),
                        "Goal".to_string(),
                        "Due".to_string(),
                        "Description".to_string(),
                        "Urg".to_string(),
                    ]),
                    sort: Some(vec!["ID-".to_string()]),
                    filter: Some("status:pending and GOAL.any:".to_string()),
                    ..Default::default()
                }),
                (FypmReports::Recurring, TaskWarriorReportConfig {
                    sort: Some(vec!["due+".to_string(), "urgency+".to_string(), "entry+".to_string()]),
                    ..Default::default()
                }),
            ]),
            uda: BTreeMap::from([
                (FypmUDAs::Style, TaskWarriorUDAConfig {
                    r#type: "string".to_string(),
                    label: "Style".to_string(),
                    values: Some(vec!["default".to_string(), "important".to_string()]),
                    default: Some("default".to_string()),
                }),
                (FypmUDAs::Type, TaskWarriorUDAConfig {
                    r#type: "string".to_string(),
                    label: "Type".to_string(),
                    values: Some(vec!["task".to_string(), "event".to_string()]),
                    default: Some("task".to_string()),
                }),
                (FypmUDAs::State, TaskWarriorUDAConfig {
                    r#type: "string".to_string(),
                    label: "State".to_string(),
                    values: Some(vec!["active".to_string(), "inactive".to_string()]),
                    default: Some("active".to_string()),
                }),
                (FypmUDAs::Mother, TaskWarriorUDAConfig {
                    r#type: "string".to_string(),
                    label: "Mother".to_string(),
                    values: Some(vec!["none".to_string()]),
                    default: Some("none".to_string()),
                }),
                (FypmUDAs::Inforelat, TaskWarriorUDAConfig {
                    r#type: "string".to_string(),
                    label: "Inforelat".to_string(),
                    values: Some(vec!["none".to_string()]),
                    default: Some("none".to_string()),
                }),
                (FypmUDAs::SeqCurrent, TaskWarriorUDAConfig {
                    r#type: "integer".to_string(),
                    label: "Current Sequence".to_string(),
                    values: None,
                    default: None,
                }),
                (FypmUDAs::SeqPrevious, TaskWarriorUDAConfig {
                    r#type: "integer".to_string(),
                    label: "Previous Sequence".to_string(),
                    values: None,
                    default: None,
                }),
                (FypmUDAs::SeqNext, TaskWarriorUDAConfig {
                    r#type: "integer".to_string(),
                    label: "Next Sequence".to_string(),
                    values: None,
                    default: None,
                }),
                (FypmUDAs::Wt, TaskWarriorUDAConfig {
                    r#type: "string".to_string(),
                    label: "WorkTime".to_string(),
                    values: Some(vec!["none".to_string()]),
                    default: Some("none".to_string()),
                }),
                (FypmUDAs::Goal, TaskWarriorUDAConfig {
                    r#type: "string".to_string(),
                    label: "Goal".to_string(),
                    values: Some(vec!["none".to_string()]),
                    default: Some("none".to_string()),
                }),
                (FypmUDAs::Alarm, TaskWarriorUDAConfig {
                    r#type: "string".to_string(),
                    label: "Alarm".to_string(),
                    values: Some(vec!["none".to_string()]),
                    default: Some("none".to_string()),
                }),
                (FypmUDAs::Effort, TaskWarriorUDAConfig {
                    r#type: "integer".to_string(),
                    label: "Effort".to_string(),
                    values: None,
                    default: None,
                }),
                (FypmUDAs::Quadrant, TaskWarriorUDAConfig {
                    r#type: "integer".to_string(),
                    label: "Quadrant".to_string(),
                    values: Some(vec!["1".to_string(), "2".to_string(), "3".to_string(), "4".to_string()]),
                    default: Some("1".to_string()),
                }),
                (FypmUDAs::Estimate, TaskWarriorUDAConfig {
                    r#type: "integer".to_string(),
                    label: "Estimate".to_string(),
                    values: None,
                    default: None,
                }),
            ]),
            urgency: BTreeMap::from([
                (FypmUrgency::Active, TaskWarriorUrgencyConfig {
                    coefficient: 1000.0,
                    scope: TaskWarriorUrgencyConfigScope::Common,
                }),
                (FypmUrgency::Tags, TaskWarriorUrgencyConfig {
                    coefficient: 0.0,
                    scope: TaskWarriorUrgencyConfigScope::Common
                }),
                (FypmUrgency::Project, TaskWarriorUrgencyConfig {
                    coefficient: 0.0,
                    scope: TaskWarriorUrgencyConfigScope::Common
                }),
                (FypmUrgency::Annotations, TaskWarriorUrgencyConfig {
                    coefficient: 0.0,
                    scope: TaskWarriorUrgencyConfigScope::Common
                }),
                (FypmUrgency::Scheduled, TaskWarriorUrgencyConfig {
                    coefficient: 20.0,
                    scope: TaskWarriorUrgencyConfigScope::Common
                }),
                (FypmUrgency::Overdue, TaskWarriorUrgencyConfig {
                    coefficient: 50.0,
                    scope: TaskWarriorUrgencyConfigScope::User { property: TaskWarriorUserScopeProperty::Tag },
                }),
                (FypmUrgency::Waiting, TaskWarriorUrgencyConfig {
                    coefficient: -10.0,
                    scope: TaskWarriorUrgencyConfigScope::User { property: TaskWarriorUserScopeProperty::Tag },
                }),
                (FypmUrgency::Template, TaskWarriorUrgencyConfig {
                    coefficient: -20.0,
                    scope: TaskWarriorUrgencyConfigScope::User { property: TaskWarriorUserScopeProperty::Tag },
                }),
                (FypmUrgency::Completed, TaskWarriorUrgencyConfig {
                    coefficient: -30.0,
                    scope: TaskWarriorUrgencyConfigScope::User { property: TaskWarriorUserScopeProperty::Tag },
                }),
                (FypmUrgency::Deleted, TaskWarriorUrgencyConfig {
                    coefficient: -50.0,
                    scope: TaskWarriorUrgencyConfigScope::User { property: TaskWarriorUserScopeProperty::Tag },
                }),
                (FypmUrgency::WtQuantify, TaskWarriorUrgencyConfig {
                    coefficient: 0.0,
                    scope: TaskWarriorUrgencyConfigScope::UDA
                }),
                (FypmUrgency::WtAllDay, TaskWarriorUrgencyConfig {
                    coefficient: -200.0,
                    scope: TaskWarriorUrgencyConfigScope::UDA
                }),
                (FypmUrgency::WtNonSched, TaskWarriorUrgencyConfig {
                    coefficient: -5.0,
                    scope: TaskWarriorUrgencyConfigScope::UDA
                }),
                (FypmUrgency::TypeSubTask, TaskWarriorUrgencyConfig {
                    coefficient: -8.0,
                    scope: TaskWarriorUrgencyConfigScope::UDA
                }),
                (FypmUrgency::TypeEssential, TaskWarriorUrgencyConfig {
                    coefficient: 5.0,
                    scope: TaskWarriorUrgencyConfigScope::UDA
                }),
                (FypmUrgency::TypeObjective, TaskWarriorUrgencyConfig {
                    coefficient: -5.0,
                    scope: TaskWarriorUrgencyConfigScope::UDA
                }),
                (FypmUrgency::TypeContinuous, TaskWarriorUrgencyConfig {
                    coefficient: 0.0,
                    scope: TaskWarriorUrgencyConfigScope::UDA
                }),
                (FypmUrgency::TypeCheck, TaskWarriorUrgencyConfig {
                    coefficient: 0.0,
                    scope: TaskWarriorUrgencyConfigScope::UDA
                }),
                (FypmUrgency::TypeEvent, TaskWarriorUrgencyConfig {
                    coefficient: -175.0,
                    scope: TaskWarriorUrgencyConfigScope::UDA
                }),
                (FypmUrgency::StyleApollonian, TaskWarriorUrgencyConfig {
                    coefficient: 3.0,
                    scope: TaskWarriorUrgencyConfigScope::UDA
                }),
                (FypmUrgency::StyleCreative, TaskWarriorUrgencyConfig {
                    coefficient: 1.0,
                    scope: TaskWarriorUrgencyConfigScope::UDA
                }),
                (FypmUrgency::StyleDionysian, TaskWarriorUrgencyConfig {
                    coefficient: -2.0,
                    scope: TaskWarriorUrgencyConfigScope::UDA
                }),
                (FypmUrgency::StyleNecessity, TaskWarriorUrgencyConfig {
                    coefficient: 5.0,
                    scope: TaskWarriorUrgencyConfigScope::UDA
                }),
                (FypmUrgency::EffortZero, TaskWarriorUrgencyConfig {
                    coefficient: 0.0,
                    scope: TaskWarriorUrgencyConfigScope::UDA
                }),
                (FypmUrgency::EffortOne, TaskWarriorUrgencyConfig {
                    coefficient: 1.0,
                    scope: TaskWarriorUrgencyConfigScope::UDA
                }),
                (FypmUrgency::EffortTwo, TaskWarriorUrgencyConfig {
                    coefficient: 2.0,
                    scope: TaskWarriorUrgencyConfigScope::UDA
                }),
                (FypmUrgency::EffortThree, TaskWarriorUrgencyConfig {
                    coefficient: 3.0,
                    scope: TaskWarriorUrgencyConfigScope::UDA
                }),
                (FypmUrgency::EffortFour, TaskWarriorUrgencyConfig {
                    coefficient: 4.0,
                    scope: TaskWarriorUrgencyConfigScope::UDA
                }),
                (FypmUrgency::EffortFive, TaskWarriorUrgencyConfig {
                    coefficient: 5.0,
                    scope: TaskWarriorUrgencyConfigScope::UDA
                }),
                (FypmUrgency::QuadrantOne, TaskWarriorUrgencyConfig {
                    coefficient: 10.0,
                    scope: TaskWarriorUrgencyConfigScope::UDA
                }),
                (FypmUrgency::QuadrantTwo, TaskWarriorUrgencyConfig {
                    coefficient: 7.0,
                    scope: TaskWarriorUrgencyConfigScope::UDA
                }),
                (FypmUrgency::QuadrantThree, TaskWarriorUrgencyConfig {
                    coefficient: 5.0,
                    scope: TaskWarriorUrgencyConfigScope::UDA
                }),
                (FypmUrgency::QuadrantNone, TaskWarriorUrgencyConfig {
                    coefficient: 0.0,
                    scope: TaskWarriorUrgencyConfigScope::UDA
                }),
                (FypmUrgency::UrgP5, TaskWarriorUrgencyConfig {
                    coefficient: 5.0,
                    scope: TaskWarriorUrgencyConfigScope::User { property: TaskWarriorUserScopeProperty::Tag },
                }),
                (FypmUrgency::UrgP10, TaskWarriorUrgencyConfig {
                    coefficient: 10.0,
                    scope: TaskWarriorUrgencyConfigScope::User { property: TaskWarriorUserScopeProperty::Tag },
                }),
                (FypmUrgency::UrgP15, TaskWarriorUrgencyConfig {
                    coefficient: 15.0,
                    scope: TaskWarriorUrgencyConfigScope::User { property: TaskWarriorUserScopeProperty::Tag },
                }),
                (FypmUrgency::UrgP20, TaskWarriorUrgencyConfig {
                    coefficient: 20.0,
                    scope: TaskWarriorUrgencyConfigScope::User { property: TaskWarriorUserScopeProperty::Tag },
                }),
                (FypmUrgency::UrgP25, TaskWarriorUrgencyConfig {
                    coefficient: 25.0,
                    scope: TaskWarriorUrgencyConfigScope::User { property: TaskWarriorUserScopeProperty::Tag },
                }),
                (FypmUrgency::UrgP30, TaskWarriorUrgencyConfig {
                    coefficient: 30.0,
                    scope: TaskWarriorUrgencyConfigScope::User { property: TaskWarriorUserScopeProperty::Tag },
                }),
                (FypmUrgency::UrgP100, TaskWarriorUrgencyConfig {
                    coefficient: 100.0,
                    scope: TaskWarriorUrgencyConfigScope::User { property: TaskWarriorUserScopeProperty::Tag },
                }),
                (FypmUrgency::UrgN5, TaskWarriorUrgencyConfig {
                    coefficient: -5.0,
                    scope: TaskWarriorUrgencyConfigScope::User { property: TaskWarriorUserScopeProperty::Tag },
                }),
                (FypmUrgency::UrgN10, TaskWarriorUrgencyConfig {
                    coefficient: -10.0,
                    scope: TaskWarriorUrgencyConfigScope::User { property: TaskWarriorUserScopeProperty::Tag },
                }),
                (FypmUrgency::UrgN15, TaskWarriorUrgencyConfig {
                    coefficient: -15.0,
                    scope: TaskWarriorUrgencyConfigScope::User { property: TaskWarriorUserScopeProperty::Tag },
                }),
                (FypmUrgency::UrgN20, TaskWarriorUrgencyConfig {
                    coefficient: -20.0,
                    scope: TaskWarriorUrgencyConfigScope::User { property: TaskWarriorUserScopeProperty::Tag },
                }),
                (FypmUrgency::UrgN25, TaskWarriorUrgencyConfig {
                    coefficient: -25.0,
                    scope: TaskWarriorUrgencyConfigScope::User { property: TaskWarriorUserScopeProperty::Tag },
                }),
                (FypmUrgency::UrgN30, TaskWarriorUrgencyConfig {
                    coefficient: -30.0,
                    scope: TaskWarriorUrgencyConfigScope::User { property: TaskWarriorUserScopeProperty::Tag },
                }),
                (FypmUrgency::UrgN100, TaskWarriorUrgencyConfig {
                    coefficient: -100.0,
                    scope: TaskWarriorUrgencyConfigScope::User { property: TaskWarriorUserScopeProperty::Tag },
                })
            ])
        }
    }
    pub fn get_defaults_btreemap(
        defaults: &FypmConfigs,
    ) -> Result<BTreeMap<String, String>, FypmError> {
        let mut hashmap: BTreeMap<String, String> = BTreeMap::new();

        // UDA
        for key in defaults.uda.keys() {
            let value = defaults.uda.get(key).unwrap();

            hashmap.insert(format!("uda.{}.type", key), value.r#type.clone());
            hashmap.insert(format!("uda.{}.label", key), value.label.clone());

            if let Some(values) = &value.values {
                hashmap.insert(format!("uda.{}.values", key), values.join(","));
            }

            if let Some(default) = &value.default {
                hashmap.insert(format!("uda.{}.default", key), default.clone());
            }
        }

        // report
        for key in defaults.report.keys() {
            let value = defaults.report.get(key).unwrap();

            if let Some(columns) = &value.columns {
                hashmap.insert(format!("report.{}.columns", key), columns.join(","));
            }

            if let Some(labels) = &value.labels {
                hashmap.insert(format!("report.{}.labels", key), labels.join(","));
            }

            if let Some(sort) = &value.sort {
                hashmap.insert(format!("report.{}.sort", key), sort.join(","));
            }

            if let Some(filter) = &value.filter {
                hashmap.insert(format!("report.{}.filter", key), filter.clone());
            }
        }

        // urgency
        for key in defaults.urgency.keys() {
            let value = defaults.urgency.get(key).unwrap();

            if value.scope == TaskWarriorUrgencyConfigScope::UDA {
                let parsed_key = key.to_string();
                let key_parts = parsed_key.split("-").collect::<Vec<&str>>();

                if key_parts.len() == 2 {
                    hashmap.insert(
                        format!("urgency.uda.{}.{}.coefficient", key_parts[0], key_parts[1]),
                        value.coefficient.to_string(),
                    );
                }
            }

            if value.scope == TaskWarriorUrgencyConfigScope::Common {
                hashmap.insert(
                    format!("urgency.{}.coefficient", key),
                    value.coefficient.to_string(),
                );
            }

            if let TaskWarriorUrgencyConfigScope::User { property } = &value.scope {
                if *property == TaskWarriorUserScopeProperty::Tag {
                    hashmap.insert(
                        format!("urgency.user.tag.{}.coefficient", key),
                        value.coefficient.to_string(),
                    );
                }
            }
        }

        Ok(hashmap)
    }

    pub fn get_config(config_file_name: &str) -> Result<FypmConfigFile, FypmError> {
        let config_file_path = Path::new(&CONFIG_PATH.to_string()).join(config_file_name);

        let config_name: String;
        if let Some(pos) = config_file_name.find(".fypm") {
            config_name = config_file_name[..pos].to_string();
        } else {
            return Err(FypmError {
                message: "Invalid config file name!".to_string(),
                kind: FypmErrorKind::InvalidInput,
            });
        }

        let file_content = fs::read_to_string(&config_file_path).unwrap();
        let config_hashmap =
            serde_ini::from_str::<BTreeMap<String, String>>(&file_content).unwrap();

        Ok(FypmConfigFile {
            name: config_name,
            map: config_hashmap,
        })
    }
    pub fn verify_config_entries(
        config: &FypmConfigFile,
        forbidden_keys: &Vec<&str>,
        allowed_keys: &Vec<&str>,
    ) -> Result<usize, FypmError> {
        let mut entries_count = 0;

        for key in config.map.keys() {
            let first_key = key.split(".").nth(0).unwrap();

            if forbidden_keys.contains(&first_key) {
                return Err(FypmError {
                    message: format!("Invalid key in {} config: {}", config.name, key),
                    kind: FypmErrorKind::InvalidConfig,
                });
            } else {
                if allowed_keys.len() == 1 && allowed_keys[0] == "*" {
                    entries_count += 1;

                    continue;
                }

                if allowed_keys.contains(&first_key) {
                    entries_count += 1;
                } else {
                    return Err(FypmError {
                        message: format!(
                            "Unknown key in {} config: {}. Are you sure if it's allowed?",
                            config.name, key
                        ),
                        kind: FypmErrorKind::InvalidConfig,
                    });
                }
            }
        }

        Ok(entries_count)
    }

    }
}
