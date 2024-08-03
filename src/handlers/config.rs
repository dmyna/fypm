use crate::utils::comments::{
    COLORS_CONFIG_COMMENT, OVERLAY_CONFIG_COMMENT, REPORT_CONFIG_COMMENT, TASK_CONFIG_COMMENT,
    UDA_CONFIG_COMMENT, URGENCY_CONFIG_COMMENT,
};
use crate::utils::constants::DEFAULT_CONFIG_FILES;
use crate::utils::enums::{FypmReports, FypmUDAs, FypmUrgency};
use crate::utils::err::{FypmError, FypmErrorKind};
use crate::utils::structs::{
    FypmConfigFile, TaskWarriorReportConfig, TaskWarriorUDAConfig, TaskWarriorUrgencyConfig,
    TaskWarriorUrgencyConfigScope, TaskWarriorUserScopeProperty,
};
use crate::CONFIG_PATH;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::{env, fs};

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

    fn create_config_defaults() -> FypmConfigs {
        FypmConfigs {
            report: BTreeMap::from([
                (
                    FypmReports::Waiting,
                    TaskWarriorReportConfig {
                        sort: Some(vec!["wait+".to_string()]),
                        ..Default::default()
                    },
                ),
                (
                    FypmReports::Next,
                    TaskWarriorReportConfig {
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
                    },
                ),
                (
                    FypmReports::List,
                    TaskWarriorReportConfig {
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
                    },
                ),
                (
                    FypmReports::All,
                    TaskWarriorReportConfig {
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
                    },
                ),
                (
                    FypmReports::Goals,
                    TaskWarriorReportConfig {
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
                    },
                ),
                (
                    FypmReports::Alarms,
                    TaskWarriorReportConfig {
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
                    },
                ),
                (
                    FypmReports::Const,
                    TaskWarriorReportConfig {
                        columns: Some(vec![
                            "uuid.short".to_string(),
                            "entry.age".to_string(),
                            "STYLE".to_string(),
                            "TYPE".to_string(),
                            "project".to_string(),
                            "tags".to_string(),
                            "WT".to_string(),
                            "GOAL".to_string(),
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
                            "WorkTime".to_string(),
                            "Goal".to_string(),
                            "Due".to_string(),
                            "ID".to_string(),
                            "Description".to_string(),
                            "Urg".to_string(),
                        ]),
                        sort: Some(vec!["due+".to_string()]),
                        ..Default::default()
                    },
                ),
                (
                    FypmReports::Recurring,
                    TaskWarriorReportConfig {
                        sort: Some(vec![
                            "due+".to_string(),
                            "urgency+".to_string(),
                            "entry+".to_string(),
                        ]),
                        ..Default::default()
                    },
                ),
            ]),
            uda: BTreeMap::from([
                (
                    FypmUDAs::Style,
                    TaskWarriorUDAConfig {
                        r#type: "string".to_string(),
                        label: "Style".to_string(),
                        values: Some(vec![
                            "Apollonian".to_string(),
                            "Dionysian".to_string(),
                            "Creative".to_string(),
                            "Necessity".to_string(),
                            "Idle".to_string(),
                        ]),
                        ..Default::default()
                    },
                ),
                (
                    FypmUDAs::Type,
                    TaskWarriorUDAConfig {
                        r#type: "string".to_string(),
                        label: "Type".to_string(),
                        values: Some(vec![
                            "Habit".to_string(),
                            "Eventual".to_string(),
                            "Objective".to_string(),
                            "Continuous".to_string(),
                            "SubTask".to_string(),
                            "Event".to_string(),
                            "Check".to_string(),
                        ]),
                        ..Default::default()
                    },
                ),
                (
                    FypmUDAs::State,
                    TaskWarriorUDAConfig {
                        r#type: "string".to_string(),
                        label: "State".to_string(),
                        values: Some(vec!["Info".to_string(), "Time".to_string()]),
                        default: Some("Time".to_string()),
                    },
                ),
                (
                    FypmUDAs::Mother,
                    TaskWarriorUDAConfig {
                        r#type: "string".to_string(),
                        label: "Mother".to_string(),
                        ..Default::default()
                    },
                ),
                (
                    FypmUDAs::Inforelat,
                    TaskWarriorUDAConfig {
                        r#type: "string".to_string(),
                        label: "Inforelat".to_string(),
                        ..Default::default()
                    },
                ),
                (
                    FypmUDAs::SeqCurrent,
                    TaskWarriorUDAConfig {
                        r#type: "string".to_string(),
                        label: "Current Sequence".to_string(),
                        ..Default::default()
                    },
                ),
                (
                    FypmUDAs::SeqPrevious,
                    TaskWarriorUDAConfig {
                        r#type: "string".to_string(),
                        label: "Previous Sequence".to_string(),
                        ..Default::default()
                    },
                ),
                (
                    FypmUDAs::SeqNext,
                    TaskWarriorUDAConfig {
                        r#type: "string".to_string(),
                        label: "Next Sequence".to_string(),
                        ..Default::default()
                    },
                ),
                (
                    FypmUDAs::Wt,
                    TaskWarriorUDAConfig {
                        r#type: "string".to_string(),
                        label: "WorkTime".to_string(),
                        values: Some(vec![
                            "Quantify!".to_string(),
                            "AllDay!".to_string(),
                            "NonSched!".to_string(),
                            "Break!".to_string(),
                        ]),
                        default: Some("NonSched!".to_string()),
                    },
                ),
                (
                    FypmUDAs::Goal,
                    TaskWarriorUDAConfig {
                        r#type: "date".to_string(),
                        label: "Goal".to_string(),
                        ..Default::default()
                    },
                ),
                (
                    FypmUDAs::Alarm,
                    TaskWarriorUDAConfig {
                        r#type: "date".to_string(),
                        label: "Alarm".to_string(),
                        ..Default::default()
                    },
                ),
                (
                    FypmUDAs::Effort,
                    TaskWarriorUDAConfig {
                        r#type: "string".to_string(),
                        label: "Effort".to_string(),
                        values: Some(vec![
                            "Inconsistent".to_string(),
                            "One".to_string(),
                            "Two".to_string(),
                            "Three".to_string(),
                            "Four".to_string(),
                            "Five".to_string(),
                            "None".to_string(),
                        ]),
                        default: Some("None".to_string()),
                    },
                ),
                (
                    FypmUDAs::Quadrant,
                    TaskWarriorUDAConfig {
                        r#type: "string".to_string(),
                        label: "Quadrant".to_string(),
                        values: Some(vec![
                            "One".to_string(),
                            "Two".to_string(),
                            "Three".to_string(),
                            "None".to_string(),
                        ]),
                        default: Some("None".to_string()),
                    },
                ),
                (
                    FypmUDAs::Estimate,
                    TaskWarriorUDAConfig {
                        r#type: "string".to_string(),
                        label: "Estimate".to_string(),
                        ..Default::default()
                    },
                ),
            ]),
            urgency: BTreeMap::from([
                (
                    FypmUrgency::Active,
                    TaskWarriorUrgencyConfig {
                        coefficient: 1000.0,
                        scope: TaskWarriorUrgencyConfigScope::Common,
                    },
                ),
                (
                    FypmUrgency::Tags,
                    TaskWarriorUrgencyConfig {
                        coefficient: 0.0,
                        scope: TaskWarriorUrgencyConfigScope::Common,
                    },
                ),
                (
                    FypmUrgency::Project,
                    TaskWarriorUrgencyConfig {
                        coefficient: 0.0,
                        scope: TaskWarriorUrgencyConfigScope::Common,
                    },
                ),
                (
                    FypmUrgency::Annotations,
                    TaskWarriorUrgencyConfig {
                        coefficient: 0.0,
                        scope: TaskWarriorUrgencyConfigScope::Common,
                    },
                ),
                (
                    FypmUrgency::Scheduled,
                    TaskWarriorUrgencyConfig {
                        coefficient: 20.0,
                        scope: TaskWarriorUrgencyConfigScope::Common,
                    },
                ),
                (
                    FypmUrgency::Overdue,
                    TaskWarriorUrgencyConfig {
                        coefficient: 50.0,
                        scope: TaskWarriorUrgencyConfigScope::User {
                            property: TaskWarriorUserScopeProperty::Tag,
                        },
                    },
                ),
                (
                    FypmUrgency::Waiting,
                    TaskWarriorUrgencyConfig {
                        coefficient: -10.0,
                        scope: TaskWarriorUrgencyConfigScope::User {
                            property: TaskWarriorUserScopeProperty::Tag,
                        },
                    },
                ),
                (
                    FypmUrgency::Template,
                    TaskWarriorUrgencyConfig {
                        coefficient: -20.0,
                        scope: TaskWarriorUrgencyConfigScope::User {
                            property: TaskWarriorUserScopeProperty::Tag,
                        },
                    },
                ),
                (
                    FypmUrgency::Completed,
                    TaskWarriorUrgencyConfig {
                        coefficient: -30.0,
                        scope: TaskWarriorUrgencyConfigScope::User {
                            property: TaskWarriorUserScopeProperty::Tag,
                        },
                    },
                ),
                (
                    FypmUrgency::Deleted,
                    TaskWarriorUrgencyConfig {
                        coefficient: -50.0,
                        scope: TaskWarriorUrgencyConfigScope::User {
                            property: TaskWarriorUserScopeProperty::Tag,
                        },
                    },
                ),
                (
                    FypmUrgency::WtQuantify,
                    TaskWarriorUrgencyConfig {
                        coefficient: 0.0,
                        scope: TaskWarriorUrgencyConfigScope::UDA,
                    },
                ),
                (
                    FypmUrgency::WtAllDay,
                    TaskWarriorUrgencyConfig {
                        coefficient: -200.0,
                        scope: TaskWarriorUrgencyConfigScope::UDA,
                    },
                ),
                (
                    FypmUrgency::WtNonSched,
                    TaskWarriorUrgencyConfig {
                        coefficient: -5.0,
                        scope: TaskWarriorUrgencyConfigScope::UDA,
                    },
                ),
                (
                    FypmUrgency::TypeSubTask,
                    TaskWarriorUrgencyConfig {
                        coefficient: -8.0,
                        scope: TaskWarriorUrgencyConfigScope::UDA,
                    },
                ),
                (
                    FypmUrgency::TypeEventual,
                    TaskWarriorUrgencyConfig {
                        coefficient: 5.0,
                        scope: TaskWarriorUrgencyConfigScope::UDA,
                    },
                ),
                (
                    FypmUrgency::TypeHabit,
                    TaskWarriorUrgencyConfig {
                        coefficient: 7.0,
                        scope: TaskWarriorUrgencyConfigScope::UDA,
                    },
                ),
                (
                    FypmUrgency::TypeObjective,
                    TaskWarriorUrgencyConfig {
                        coefficient: -5.0,
                        scope: TaskWarriorUrgencyConfigScope::UDA,
                    },
                ),
                (
                    FypmUrgency::TypeContinuous,
                    TaskWarriorUrgencyConfig {
                        coefficient: 0.0,
                        scope: TaskWarriorUrgencyConfigScope::UDA,
                    },
                ),
                (
                    FypmUrgency::TypeCheck,
                    TaskWarriorUrgencyConfig {
                        coefficient: 0.0,
                        scope: TaskWarriorUrgencyConfigScope::UDA,
                    },
                ),
                (
                    FypmUrgency::TypeEvent,
                    TaskWarriorUrgencyConfig {
                        coefficient: -175.0,
                        scope: TaskWarriorUrgencyConfigScope::UDA,
                    },
                ),
                (
                    FypmUrgency::StyleApollonian,
                    TaskWarriorUrgencyConfig {
                        coefficient: 3.0,
                        scope: TaskWarriorUrgencyConfigScope::UDA,
                    },
                ),
                (
                    FypmUrgency::StyleCreative,
                    TaskWarriorUrgencyConfig {
                        coefficient: 1.0,
                        scope: TaskWarriorUrgencyConfigScope::UDA,
                    },
                ),
                (
                    FypmUrgency::StyleDionysian,
                    TaskWarriorUrgencyConfig {
                        coefficient: -2.0,
                        scope: TaskWarriorUrgencyConfigScope::UDA,
                    },
                ),
                (
                    FypmUrgency::StyleNecessity,
                    TaskWarriorUrgencyConfig {
                        coefficient: 5.0,
                        scope: TaskWarriorUrgencyConfigScope::UDA,
                    },
                ),
                (
                    FypmUrgency::EffortZero,
                    TaskWarriorUrgencyConfig {
                        coefficient: 0.0,
                        scope: TaskWarriorUrgencyConfigScope::UDA,
                    },
                ),
                (
                    FypmUrgency::EffortOne,
                    TaskWarriorUrgencyConfig {
                        coefficient: 1.0,
                        scope: TaskWarriorUrgencyConfigScope::UDA,
                    },
                ),
                (
                    FypmUrgency::EffortTwo,
                    TaskWarriorUrgencyConfig {
                        coefficient: 2.0,
                        scope: TaskWarriorUrgencyConfigScope::UDA,
                    },
                ),
                (
                    FypmUrgency::EffortThree,
                    TaskWarriorUrgencyConfig {
                        coefficient: 3.0,
                        scope: TaskWarriorUrgencyConfigScope::UDA,
                    },
                ),
                (
                    FypmUrgency::EffortFour,
                    TaskWarriorUrgencyConfig {
                        coefficient: 4.0,
                        scope: TaskWarriorUrgencyConfigScope::UDA,
                    },
                ),
                (
                    FypmUrgency::EffortFive,
                    TaskWarriorUrgencyConfig {
                        coefficient: 5.0,
                        scope: TaskWarriorUrgencyConfigScope::UDA,
                    },
                ),
                (
                    FypmUrgency::QuadrantOne,
                    TaskWarriorUrgencyConfig {
                        coefficient: 10.0,
                        scope: TaskWarriorUrgencyConfigScope::UDA,
                    },
                ),
                (
                    FypmUrgency::QuadrantTwo,
                    TaskWarriorUrgencyConfig {
                        coefficient: 7.0,
                        scope: TaskWarriorUrgencyConfigScope::UDA,
                    },
                ),
                (
                    FypmUrgency::QuadrantThree,
                    TaskWarriorUrgencyConfig {
                        coefficient: 5.0,
                        scope: TaskWarriorUrgencyConfigScope::UDA,
                    },
                ),
                (
                    FypmUrgency::QuadrantNone,
                    TaskWarriorUrgencyConfig {
                        coefficient: 0.0,
                        scope: TaskWarriorUrgencyConfigScope::UDA,
                    },
                ),
                (
                    FypmUrgency::UrgP5,
                    TaskWarriorUrgencyConfig {
                        coefficient: 5.0,
                        scope: TaskWarriorUrgencyConfigScope::User {
                            property: TaskWarriorUserScopeProperty::Tag,
                        },
                    },
                ),
                (
                    FypmUrgency::UrgP10,
                    TaskWarriorUrgencyConfig {
                        coefficient: 10.0,
                        scope: TaskWarriorUrgencyConfigScope::User {
                            property: TaskWarriorUserScopeProperty::Tag,
                        },
                    },
                ),
                (
                    FypmUrgency::UrgP15,
                    TaskWarriorUrgencyConfig {
                        coefficient: 15.0,
                        scope: TaskWarriorUrgencyConfigScope::User {
                            property: TaskWarriorUserScopeProperty::Tag,
                        },
                    },
                ),
                (
                    FypmUrgency::UrgP20,
                    TaskWarriorUrgencyConfig {
                        coefficient: 20.0,
                        scope: TaskWarriorUrgencyConfigScope::User {
                            property: TaskWarriorUserScopeProperty::Tag,
                        },
                    },
                ),
                (
                    FypmUrgency::UrgP25,
                    TaskWarriorUrgencyConfig {
                        coefficient: 25.0,
                        scope: TaskWarriorUrgencyConfigScope::User {
                            property: TaskWarriorUserScopeProperty::Tag,
                        },
                    },
                ),
                (
                    FypmUrgency::UrgP30,
                    TaskWarriorUrgencyConfig {
                        coefficient: 30.0,
                        scope: TaskWarriorUrgencyConfigScope::User {
                            property: TaskWarriorUserScopeProperty::Tag,
                        },
                    },
                ),
                (
                    FypmUrgency::UrgP100,
                    TaskWarriorUrgencyConfig {
                        coefficient: 100.0,
                        scope: TaskWarriorUrgencyConfigScope::User {
                            property: TaskWarriorUserScopeProperty::Tag,
                        },
                    },
                ),
                (
                    FypmUrgency::UrgN5,
                    TaskWarriorUrgencyConfig {
                        coefficient: -5.0,
                        scope: TaskWarriorUrgencyConfigScope::User {
                            property: TaskWarriorUserScopeProperty::Tag,
                        },
                    },
                ),
                (
                    FypmUrgency::UrgN10,
                    TaskWarriorUrgencyConfig {
                        coefficient: -10.0,
                        scope: TaskWarriorUrgencyConfigScope::User {
                            property: TaskWarriorUserScopeProperty::Tag,
                        },
                    },
                ),
                (
                    FypmUrgency::UrgN15,
                    TaskWarriorUrgencyConfig {
                        coefficient: -15.0,
                        scope: TaskWarriorUrgencyConfigScope::User {
                            property: TaskWarriorUserScopeProperty::Tag,
                        },
                    },
                ),
                (
                    FypmUrgency::UrgN20,
                    TaskWarriorUrgencyConfig {
                        coefficient: -20.0,
                        scope: TaskWarriorUrgencyConfigScope::User {
                            property: TaskWarriorUserScopeProperty::Tag,
                        },
                    },
                ),
                (
                    FypmUrgency::UrgN25,
                    TaskWarriorUrgencyConfig {
                        coefficient: -25.0,
                        scope: TaskWarriorUrgencyConfigScope::User {
                            property: TaskWarriorUserScopeProperty::Tag,
                        },
                    },
                ),
                (
                    FypmUrgency::UrgN30,
                    TaskWarriorUrgencyConfig {
                        coefficient: -30.0,
                        scope: TaskWarriorUrgencyConfigScope::User {
                            property: TaskWarriorUserScopeProperty::Tag,
                        },
                    },
                ),
                (
                    FypmUrgency::UrgN100,
                    TaskWarriorUrgencyConfig {
                        coefficient: -100.0,
                        scope: TaskWarriorUrgencyConfigScope::User {
                            property: TaskWarriorUserScopeProperty::Tag,
                        },
                    },
                ),
            ]),
        }
    }
    fn get_defaults_btreemap(
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

    fn get_config(config_file_name: &str) -> Result<FypmConfigFile, FypmError> {
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
    fn verify_config_entries(
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

    fn mount_taskrc() -> Result<BTreeMap<String, String>, FypmError> {
        let mut configs_map = BTreeMap::new();

        // Taskwarrior user-defined and fypm defaults
        {
            let taskwarrior_configs = ConfigHandler::get_config(DEFAULT_CONFIG_FILES[0]).unwrap();
            ConfigHandler::verify_config_entries(&taskwarrior_configs, &vec![], &vec!["*"])
                .unwrap();

            configs_map.extend(taskwarrior_configs.map);

            let defaults = ConfigHandler::create_config_defaults();
            let defaults_map = ConfigHandler::get_defaults_btreemap(&defaults).unwrap();

            configs_map.extend(defaults_map);
        }

        {
            //. Put Worktime config here
        }

        // General user-defined configs
        {
            let uda_configs = ConfigHandler::get_config(DEFAULT_CONFIG_FILES[1]).unwrap();
            ConfigHandler::verify_config_entries(
                &uda_configs,
                &vec!["report", "urgency", "color"],
                &vec!["uda"],
            )
            .unwrap();

            configs_map.extend(uda_configs.map);

            let report_configs = ConfigHandler::get_config(DEFAULT_CONFIG_FILES[2]).unwrap();
            ConfigHandler::verify_config_entries(
                &report_configs,
                &vec!["uda", "urgency", "color"],
                &vec!["report"],
            )
            .unwrap();

            configs_map.extend(report_configs.map);

            let urgency_configs = ConfigHandler::get_config(DEFAULT_CONFIG_FILES[3]).unwrap();
            ConfigHandler::verify_config_entries(
                &urgency_configs,
                &vec!["uda", "report", "color"],
                &vec!["urgency"],
            )
            .unwrap();

            configs_map.extend(urgency_configs.map);

            let color_configs = ConfigHandler::get_config(DEFAULT_CONFIG_FILES[4]).unwrap();
            ConfigHandler::verify_config_entries(
                &color_configs,
                &vec!["uda", "report", "urgency"],
                &vec!["color"],
            )
            .unwrap();

            configs_map.extend(color_configs.map);
        }

        // Overlay configs
        {
            let overlay_configs = ConfigHandler::get_config(DEFAULT_CONFIG_FILES[5]).unwrap();
            ConfigHandler::verify_config_entries(&overlay_configs, &vec![], &vec!["*"]).unwrap();

            for key in overlay_configs.map.keys() {
                if configs_map.contains_key(key) {
                    configs_map.insert(key.clone(), overlay_configs.map.get(key).unwrap().clone());
                } else {
                    Err(FypmError {
                        message: format!("The key {} is not present in any config! Are you trying to overwrite the wind?", key),
                        kind: FypmErrorKind::InvalidConfig,
                    })?;
                }
            }
        }

        Ok(configs_map)
    }
    pub fn handle_config() -> Result<(), FypmError> {
        let taskrc_env = env::var("TASKRC").unwrap_or("".to_string());

        let taskrc_path = match taskrc_env.as_str() {
            "" => dirs::home_dir()
                .unwrap()
                .join(".taskrc")
                .to_string_lossy()
                .to_string(),
            _ => taskrc_env,
        };

        let configs_map = ConfigHandler::mount_taskrc().unwrap();

        let parsed_configs = serde_ini::to_string::<BTreeMap<String, String>>(&configs_map)
            .expect("There's a problem in default configs!");

        match fs::metadata(&taskrc_path) {
            Ok(metadata) => {
                if metadata.is_file() {
                    let mut file = File::open(&taskrc_path).unwrap();

                    let mut content = String::new();
                    file.read_to_string(&mut content).unwrap();

                    if content.trim() == parsed_configs.trim() {
                        return Ok(());
                    }
                } else {
                    return Err(FypmError {
                        message: "The .taskrc is not a file!".to_string(),
                        kind: FypmErrorKind::InvalidConfig,
                    });
                }
            }
            _ => return Ok(()),
        }

        fs::write(&taskrc_path, parsed_configs).unwrap();

        Ok(())
    }
}
