use crate::utils::constants::DEFAULT_CONFIG_FILES;
use crate::utils::err::{FypmError, FypmErrorKind};
use crate::utils::structs::{
    TaskWarriorReportConfig, TaskWarriorUDAConfig, TaskWarriorUrgencyConfig,
    TaskWarriorUrgencyConfigScope, TaskWarriorUserScopeProperty,
};
use crate::CONFIG_PATH;
use std::fs::{self, ReadDir};
use std::path::Path;

pub struct FypmReports {
    waiting: TaskWarriorReportConfig,
    next: TaskWarriorReportConfig,
    list: TaskWarriorReportConfig,
    all: TaskWarriorReportConfig,
    blist: TaskWarriorReportConfig,
    wlist: TaskWarriorReportConfig,
    goals: TaskWarriorReportConfig,
    alarms: TaskWarriorReportConfig,
    all_goals: TaskWarriorReportConfig,
    r#const: TaskWarriorReportConfig,
    recurring: TaskWarriorReportConfig,
}
pub struct FypmUdas {
    style: TaskWarriorUDAConfig,
    r#type: TaskWarriorUDAConfig,
    state: TaskWarriorUDAConfig,
    mother: TaskWarriorUDAConfig,
    inforelat: TaskWarriorUDAConfig,
    seq_current: TaskWarriorUDAConfig,
    seq_previous: TaskWarriorUDAConfig,
    seq_next: TaskWarriorUDAConfig,
    wt: TaskWarriorUDAConfig,
    goal: TaskWarriorUDAConfig,
    alarm: TaskWarriorUDAConfig,
    effort: TaskWarriorUDAConfig,
    quadrant: TaskWarriorUDAConfig,
    estimate: TaskWarriorUDAConfig,
}
pub struct FypmUrgency {
    // General
    active: TaskWarriorUrgencyConfig,
    tags: TaskWarriorUrgencyConfig,
    project: TaskWarriorUrgencyConfig,
    annotations: TaskWarriorUrgencyConfig,
    scheduled: TaskWarriorUrgencyConfig,

    // Virtual Tags
    overdue: TaskWarriorUrgencyConfig,
    waiting: TaskWarriorUrgencyConfig,
    template: TaskWarriorUrgencyConfig,
    completed: TaskWarriorUrgencyConfig,
    deleted: TaskWarriorUrgencyConfig,

    // WorkTime
    wt_quantify: TaskWarriorUrgencyConfig,
    wt_allday: TaskWarriorUrgencyConfig,
    wt_nonsched: TaskWarriorUrgencyConfig,

    // Type
    type_subtask: TaskWarriorUrgencyConfig,
    type_essential: TaskWarriorUrgencyConfig,
    type_objective: TaskWarriorUrgencyConfig,
    type_continuous: TaskWarriorUrgencyConfig,
    type_check: TaskWarriorUrgencyConfig,
    type_event: TaskWarriorUrgencyConfig,

    // Style
    style_apollonian: TaskWarriorUrgencyConfig,
    style_creative: TaskWarriorUrgencyConfig,
    style_dionysian: TaskWarriorUrgencyConfig,
    style_necessity: TaskWarriorUrgencyConfig,

    // Effort
    effort_zero: TaskWarriorUrgencyConfig,
    effort_one: TaskWarriorUrgencyConfig,
    effort_two: TaskWarriorUrgencyConfig,
    effort_three: TaskWarriorUrgencyConfig,
    effort_four: TaskWarriorUrgencyConfig,
    effort_five: TaskWarriorUrgencyConfig,

    // Quadrant
    quadrant_one: TaskWarriorUrgencyConfig,
    quadrant_two: TaskWarriorUrgencyConfig,
    quadrant_three: TaskWarriorUrgencyConfig,
    quadrant_none: TaskWarriorUrgencyConfig,

    // Urgency Increment
    urg_p5: TaskWarriorUrgencyConfig,
    urg_p10: TaskWarriorUrgencyConfig,
    urg_p15: TaskWarriorUrgencyConfig,
    urg_p20: TaskWarriorUrgencyConfig,
    urg_p25: TaskWarriorUrgencyConfig,
    urg_p30: TaskWarriorUrgencyConfig,
    urg_p100: TaskWarriorUrgencyConfig,
    urg_n5: TaskWarriorUrgencyConfig,
    urg_n10: TaskWarriorUrgencyConfig,
    urg_n15: TaskWarriorUrgencyConfig,
    urg_n20: TaskWarriorUrgencyConfig,
    urg_n25: TaskWarriorUrgencyConfig,
    urg_n30: TaskWarriorUrgencyConfig,
    urg_n100: TaskWarriorUrgencyConfig,
}
pub struct FypmConfigs {
    pub report: FypmReports,
    pub uda: FypmUdas,
    pub urgency: FypmUrgency,
}

pub struct ConfigHandler;
impl ConfigHandler {
    pub fn ensure_config_path() -> Result<(), FypmError> {
        fs::create_dir_all(CONFIG_PATH.to_string()).unwrap();

        Ok(())
    }
    pub fn ensure_config_files() -> Result<(), FypmError> {
        for config_file in DEFAULT_CONFIG_FILES {
            let file_path = Path::new(&CONFIG_PATH.to_string()).join(config_file);

            if !file_path.exists() {
                fs::File::create(file_path).unwrap();
            }
        }

        Ok(())
    }
    pub fn create_config_defaults() -> FypmConfigs {
        FypmConfigs {
            report: FypmReports {
                waiting: TaskWarriorReportConfig {
                    sort: Some(vec!["wait+".to_string()]),
                    ..Default::default()
                },
                next: TaskWarriorReportConfig {
                    columns: Some(vec![
                        "entry.age".to_string(),
                        "STYLE".to_string(),
                        "TYPE".to_string(),
                        "WT".to_string(),
                        "estimate".to_string(),
                        "due.relative".to_string(),
                        "id".to_string(),
                        "project".to_string(),
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
                        "ID".to_string(),
                        "Project".to_string(),
                        "Description".to_string(),
                        "Urg".to_string(),
                    ]),
                    sort: Some(vec!["urgency-".to_string(), "ALARM-".to_string()]),
                    ..Default::default()
                },
                list: TaskWarriorReportConfig {
                    columns: Some(vec![
                        "id".to_string(),
                        "entry.age".to_string(),
                        "STYLE".to_string(),
                        "TYPE".to_string(),
                        "project".to_string(),
                        "tags".to_string(),
                        "recur".to_string(),
                        "WT".to_string(),
                        "GOAL".to_string(),
                        "scheduled.countdown".to_string(),
                        "id".to_string(),
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
                        "Tag".to_string(),
                        "Recur".to_string(),
                        "WorkTime".to_string(),
                        "Goal".to_string(),
                        "Sched".to_string(),
                        "ID".to_string(),
                        "Due".to_string(),
                        "Description".to_string(),
                        "Urg".to_string(),
                    ]),
                    sort: Some(vec!["urgency+".to_string()]),
                    filter: Some("status:pending -WAITING -Ghost".to_string()),
                    ..Default::default()
                },
                all: TaskWarriorReportConfig {
                    columns: Some(vec![
                        "uuid.short".to_string(),
                        "id".to_string(),
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
                        "description".to_string(),
                        "urgency".to_string(),
                    ]),
                    labels: Some(vec![
                        "UUID".to_string(),
                        "ID".to_string(),
                        "Age".to_string(),
                        "Style".to_string(),
                        "Type".to_string(),
                        "Project".to_string(),
                        "Tag".to_string(),
                        "Recur".to_string(),
                        "WorkTime".to_string(),
                        "Goal".to_string(),
                        "Sched".to_string(),
                        "Due".to_string(),
                        "Description".to_string(),
                        "Urg".to_string(),
                    ]),
                    sort: Some(vec!["urgency+".to_string()]),
                    ..Default::default()
                },
                blist: TaskWarriorReportConfig {
                    columns: Some(vec![
                        "id".to_string(),
                        "status.short".to_string(),
                        "STYLE".to_string(),
                        "TYPE".to_string(),
                        "project".to_string(),
                        "tags".to_string(),
                        "WT".to_string(),
                        "GOAL".to_string(),
                        "wait.relative".to_string(),
                        "due.relative".to_string(),
                        "description".to_string(),
                        "urgency".to_string(),
                    ]),
                    filter: Some("WT.not:NonSched! and -Ghost and -DELETED and -PARENT and -COMPLETED and TYPE.not:Event".to_string()),
                    sort: Some(vec!["urgency-".to_string()]),
                    ..Default::default()
                },
                wlist: TaskWarriorReportConfig {
                    columns: Some(vec![
                        "id".to_string(),
                        "status.short".to_string(),
                        "STYLE".to_string(),
                        "TYPE".to_string(),
                        "project".to_string(),
                        "tags".to_string(),
                        "WT".to_string(),
                        "GOAL".to_string(),
                        "wait.relative".to_string(),
                        "due.relative".to_string(),
                        "description".to_string(),
                        "urgency".to_string(),
                    ]),
                    labels: Some(vec![
                        "ID".to_string(),
                        "S".to_string(),
                        "Style".to_string(),
                        "Type".to_string(),
                        "Project".to_string(),
                        "Tags".to_string(),
                        "Worktime".to_string(),
                        "Goal".to_string(),
                        "Wait".to_string(),
                        "Due".to_string(),
                        "Desc".to_string(),
                        "Urg".to_string(),
                    ]),
                    filter: Some("((+ACTIVE or +OVERDUE or +Divisory or (((due:today or due.after:today) and due.before:tomorrow) and (WT:Quantify or WT:NonSched)) or ((WT:Calm or WT:AllDay) and ((+TODAY and +INSTANCE) or (GOAL.after:2024-05-01 and GOAL.before:now and TYPE:Objective) or (ALARM.after:now and ALARM.before:18:00)))) and status:pending) or (((due:today or due.after:today) and due.before:tomorrow) and WT:AllDay and (status.not:recurring and status.not:waiting))".to_string()),
                    sort: Some(vec!["urgency-".to_string()]),
                    ..Default::default()
                },
                goals: TaskWarriorReportConfig {
                    columns: Some(vec![
                        "id".to_string(),
                        "entry.age".to_string(),
                        "TYPE".to_string(),
                        "project".to_string(),
                        "WT".to_string(),
                        "GOAL".to_string(),
                        "due.relative".to_string(),
                        "description".to_string(),
                        "urgency".to_string(),
                    ]),
                    labels: Some(vec![
                        "ID".to_string(),
                        "Age".to_string(),
                        "Type".to_string(),
                        "Project".to_string(),
                        "WorkTime".to_string(),
                        "Goal".to_string(),
                        "Due".to_string(),
                        "Description".to_string(),
                        "Urg".to_string(),
                    ]),
                    sort: Some(vec!["GOAL+".to_string()]),
                    filter: Some("status:pending and GOAL.any:".to_string()),
                    ..Default::default()
                },
                alarms: TaskWarriorReportConfig {
                    columns: Some(vec![
                        "id".to_string(),
                        "entry.age".to_string(),
                        "STYLE".to_string(),
                        "TYPE".to_string(),
                        "project".to_string(),
                        "tags".to_string(),
                        "WT".to_string(),
                        "GOAL".to_string(),
                        "ALARM.relative".to_string(),
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
                        "Tag".to_string(),
                        "WorkTime".to_string(),
                        "Goal".to_string(),
                        "Alarm".to_string(),
                        "Due".to_string(),
                        "Description".to_string(),
                        "Urg".to_string(),
                    ]),
                    sort: Some(vec!["ALARM+".to_string()]),
                    filter: Some("status:pending and ALARM.any:".to_string()),
                    ..Default::default()
                },
                all_goals: TaskWarriorReportConfig {
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
                        "Tag".to_string(),
                        "WorkTime".to_string(),
                        "Goal".to_string(),
                        "Due".to_string(),
                        "Description".to_string(),
                        "Urg".to_string(),
                    ]),
                    sort: Some(vec!["GOAL+".to_string()]),
                    filter: Some("status:pending and GOAL.any:".to_string()),
                    ..Default::default()
                },
                r#const: TaskWarriorReportConfig {
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
                        "Tag".to_string(),
                        "WorkTime".to_string(),
                        "Goal".to_string(),
                        "Due".to_string(),
                        "Description".to_string(),
                        "Urg".to_string(),
                    ]),
                    sort: Some(vec!["ID-".to_string()]),
                    filter: Some("status:pending and GOAL.any:".to_string()),
                    ..Default::default()
                },
                recurring: TaskWarriorReportConfig {
                    columns: Some(vec![
                        "id".to_string(),
                        "entry.age".to_string(),
                        "STYLE".to_string(),
                        "TYPE".to_string(),
                        "project".to_string(),
                        "tags".to_string(),
                        "WT".to_string(),
                        "GOAL".to_string(),
                        "recur".to_string(),
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
                        "Tag".to_string(),
                        "WorkTime".to_string(),
                        "Goal".to_string(),
                        "Recur".to_string(),
                        "Due".to_string(),
                        "Description".to_string(),
                        "Urg".to_string(),
                    ]),
                    sort: Some(vec!["recur+".to_string()]),
                    filter: Some("status:pending and recur.any:".to_string()),
                    ..Default::default()
                },
            },
            uda: FypmUdas {
                style: TaskWarriorUDAConfig {
                    r#type: "string".to_string(),
                    label: "Style".to_string(),
                    values: Some(vec!["default".to_string(), "important".to_string()]),
                    default: Some("default".to_string()),
                },
                r#type: TaskWarriorUDAConfig {
                    r#type: "string".to_string(),
                    label: "Type".to_string(),
                    values: Some(vec!["task".to_string(), "event".to_string()]),
                    default: Some("task".to_string()),
                },
                state: TaskWarriorUDAConfig {
                    r#type: "string".to_string(),
                    label: "State".to_string(),
                    values: Some(vec!["active".to_string(), "inactive".to_string()]),
                    default: Some("active".to_string()),
                },
                mother: TaskWarriorUDAConfig {
                    r#type: "string".to_string(),
                    label: "Mother".to_string(),
                    values: Some(vec!["none".to_string()]),
                    default: Some("none".to_string()),
                },
                inforelat: TaskWarriorUDAConfig {
                    r#type: "string".to_string(),
                    label: "Inforelat".to_string(),
                    values: Some(vec!["none".to_string()]),
                    default: Some("none".to_string()),
                },
                seq_current: TaskWarriorUDAConfig {
                    r#type: "integer".to_string(),
                    label: "Current Sequence".to_string(),
                    values: None,
                    default: None,
                },
                seq_previous: TaskWarriorUDAConfig {
                    r#type: "integer".to_string(),
                    label: "Previous Sequence".to_string(),
                    values: None,
                    default: None,
                },
                seq_next: TaskWarriorUDAConfig {
                    r#type: "integer".to_string(),
                    label: "Next Sequence".to_string(),
                    values: None,
                    default: None,
                },
                wt: TaskWarriorUDAConfig {
                    r#type: "string".to_string(),
                    label: "WorkTime".to_string(),
                    values: Some(vec!["none".to_string()]),
                    default: Some("none".to_string()),
                },
                goal: TaskWarriorUDAConfig {
                    r#type: "string".to_string(),
                    label: "Goal".to_string(),
                    values: Some(vec!["none".to_string()]),
                    default: Some("none".to_string()),
                },
                alarm: TaskWarriorUDAConfig {
                    r#type: "string".to_string(),
                    label: "Alarm".to_string(),
                    values: Some(vec!["none".to_string()]),
                    default: Some("none".to_string()),
                },
                effort: TaskWarriorUDAConfig {
                    r#type: "integer".to_string(),
                    label: "Effort".to_string(),
                    values: None,
                    default: None,
                },
                quadrant: TaskWarriorUDAConfig {
                    r#type: "integer".to_string(),
                    label: "Quadrant".to_string(),
                    values: Some(vec!["1".to_string(), "2".to_string(), "3".to_string(), "4".to_string()]),
                    default: Some("1".to_string()),
                },
                estimate: TaskWarriorUDAConfig {
                    r#type: "integer".to_string(),
                    label: "Estimate".to_string(),
                    values: None,
                    default: None,
                },
            },
            urgency: FypmUrgency {
                active: TaskWarriorUrgencyConfig {
                    coefficient: 1000.0,
                    scope: TaskWarriorUrgencyConfigScope::Common,
                },
                tags: TaskWarriorUrgencyConfig {
                    coefficient: 0.0,
                    scope: TaskWarriorUrgencyConfigScope::Common,
                },
                project: TaskWarriorUrgencyConfig {
                    coefficient: 0.0,
                    scope: TaskWarriorUrgencyConfigScope::Common,
                },
                annotations: TaskWarriorUrgencyConfig {
                    coefficient: 0.0,
                    scope: TaskWarriorUrgencyConfigScope::Common,
                },
                scheduled: TaskWarriorUrgencyConfig {
                    coefficient: 20.0,
                    scope: TaskWarriorUrgencyConfigScope::Common,
                },

                // Virtual tags
                overdue: TaskWarriorUrgencyConfig {
                    coefficient: 50.0,
                    scope: TaskWarriorUrgencyConfigScope::User { property: TaskWarriorUserScopeProperty::Tag },
                },
                waiting: TaskWarriorUrgencyConfig {
                    coefficient: -10.0,
                    scope: TaskWarriorUrgencyConfigScope::User { property: TaskWarriorUserScopeProperty::Tag },
                },
                template: TaskWarriorUrgencyConfig {
                    coefficient: -20.0,
                    scope: TaskWarriorUrgencyConfigScope::User { property: TaskWarriorUserScopeProperty::Tag },
                },
                completed: TaskWarriorUrgencyConfig {
                    coefficient: -30.0,
                    scope: TaskWarriorUrgencyConfigScope::User { property: TaskWarriorUserScopeProperty::Tag },
                },
                deleted: TaskWarriorUrgencyConfig {
                    coefficient: -50.0,
                    scope: TaskWarriorUrgencyConfigScope::User { property: TaskWarriorUserScopeProperty::Tag },
                },

                // WorkTime
                wt_quantify: TaskWarriorUrgencyConfig {
                    coefficient: 0.0,
                    scope: TaskWarriorUrgencyConfigScope::UDA,
                },
                wt_allday: TaskWarriorUrgencyConfig {
                    coefficient: -200.0,
                    scope: TaskWarriorUrgencyConfigScope::UDA,
                },
                wt_nonsched: TaskWarriorUrgencyConfig {
                    coefficient: -5.0,
                    scope: TaskWarriorUrgencyConfigScope::UDA,
                },

                // Type
                type_subtask: TaskWarriorUrgencyConfig {
                    coefficient: -8.0,
                    scope: TaskWarriorUrgencyConfigScope::UDA,
                },
                type_essential: TaskWarriorUrgencyConfig {
                    coefficient: 5.0,
                    scope: TaskWarriorUrgencyConfigScope::UDA,
                },
                type_objective: TaskWarriorUrgencyConfig {
                    coefficient: -5.0,
                    scope: TaskWarriorUrgencyConfigScope::UDA,
                },
                type_continuous: TaskWarriorUrgencyConfig {
                    coefficient: 0.0,
                    scope: TaskWarriorUrgencyConfigScope::UDA,
                },
                type_check: TaskWarriorUrgencyConfig {
                    coefficient: 0.0,
                    scope: TaskWarriorUrgencyConfigScope::UDA,
                },
                type_event: TaskWarriorUrgencyConfig {
                    coefficient: -175.0,
                    scope: TaskWarriorUrgencyConfigScope::UDA,
                },

                // Style
                style_apollonian: TaskWarriorUrgencyConfig {
                    coefficient: 3.0,
                    scope: TaskWarriorUrgencyConfigScope::UDA,
                },
                style_creative: TaskWarriorUrgencyConfig {
                    coefficient: 1.0,
                    scope: TaskWarriorUrgencyConfigScope::UDA,
                },
                style_dionysian: TaskWarriorUrgencyConfig {
                    coefficient: -2.0,
                    scope: TaskWarriorUrgencyConfigScope::UDA,
                },
                style_necessity: TaskWarriorUrgencyConfig {
                    coefficient: 5.0,
                    scope: TaskWarriorUrgencyConfigScope::UDA,
                },

                // Effort
                effort_zero: TaskWarriorUrgencyConfig {
                    coefficient: 0.0,
                    scope: TaskWarriorUrgencyConfigScope::UDA,
                },
                effort_one: TaskWarriorUrgencyConfig {
                    coefficient: 1.0,
                    scope: TaskWarriorUrgencyConfigScope::UDA,
                },
                effort_two: TaskWarriorUrgencyConfig {
                    coefficient: 2.0,
                    scope: TaskWarriorUrgencyConfigScope::UDA,
                },
                effort_three: TaskWarriorUrgencyConfig {
                    coefficient: 3.0,
                    scope: TaskWarriorUrgencyConfigScope::UDA,
                },
                effort_four: TaskWarriorUrgencyConfig {
                    coefficient: 4.0,
                    scope: TaskWarriorUrgencyConfigScope::UDA,
                },
                effort_five: TaskWarriorUrgencyConfig {
                    coefficient: 5.0,
                    scope: TaskWarriorUrgencyConfigScope::UDA,
                },

                // Quadrant
                quadrant_one: TaskWarriorUrgencyConfig {
                    coefficient: 10.0,
                    scope: TaskWarriorUrgencyConfigScope::UDA,
                },
                quadrant_two: TaskWarriorUrgencyConfig {
                    coefficient: 7.0,
                    scope: TaskWarriorUrgencyConfigScope::UDA,
                },
                quadrant_three: TaskWarriorUrgencyConfig {
                    coefficient: 5.0,
                    scope: TaskWarriorUrgencyConfigScope::UDA,
                },
                quadrant_none: TaskWarriorUrgencyConfig {
                    coefficient: 0.0,
                    scope: TaskWarriorUrgencyConfigScope::UDA,
                },

                // Urgency Increment
                urg_p5: TaskWarriorUrgencyConfig {
                    coefficient: 5.0,
                    scope: TaskWarriorUrgencyConfigScope::User { property: TaskWarriorUserScopeProperty::Tag },
                },
                urg_p10: TaskWarriorUrgencyConfig {
                    coefficient: 10.0,
                    scope: TaskWarriorUrgencyConfigScope::User { property: TaskWarriorUserScopeProperty::Tag },
                },
                urg_p15: TaskWarriorUrgencyConfig {
                    coefficient: 15.0,
                    scope: TaskWarriorUrgencyConfigScope::User { property: TaskWarriorUserScopeProperty::Tag },
                },
                urg_p20: TaskWarriorUrgencyConfig {
                    coefficient: 20.0,
                    scope: TaskWarriorUrgencyConfigScope::User { property: TaskWarriorUserScopeProperty::Tag },
                },
                urg_p25: TaskWarriorUrgencyConfig {
                    coefficient: 25.0,
                    scope: TaskWarriorUrgencyConfigScope::User { property: TaskWarriorUserScopeProperty::Tag },
                },
                urg_p30: TaskWarriorUrgencyConfig {
                    coefficient: 30.0,
                    scope: TaskWarriorUrgencyConfigScope::User { property: TaskWarriorUserScopeProperty::Tag },
                },
                urg_p100: TaskWarriorUrgencyConfig {
                    coefficient: 100.0,
                    scope: TaskWarriorUrgencyConfigScope::User { property: TaskWarriorUserScopeProperty::Tag },
                },
                urg_n5: TaskWarriorUrgencyConfig {
                    coefficient: -5.0,
                    scope: TaskWarriorUrgencyConfigScope::User { property: TaskWarriorUserScopeProperty::Tag },
                },
                urg_n10: TaskWarriorUrgencyConfig {
                    coefficient: -10.0,
                    scope: TaskWarriorUrgencyConfigScope::User { property: TaskWarriorUserScopeProperty::Tag },
                },
                urg_n15: TaskWarriorUrgencyConfig {
                    coefficient: -15.0,
                    scope: TaskWarriorUrgencyConfigScope::User { property: TaskWarriorUserScopeProperty::Tag },
                },
                urg_n20: TaskWarriorUrgencyConfig {
                    coefficient: -20.0,
                    scope: TaskWarriorUrgencyConfigScope::User { property: TaskWarriorUserScopeProperty::Tag },
                },
                urg_n25: TaskWarriorUrgencyConfig {
                    coefficient: -25.0,
                    scope: TaskWarriorUrgencyConfigScope::User { property: TaskWarriorUserScopeProperty::Tag },
                },
                urg_n30: TaskWarriorUrgencyConfig {
                    coefficient: -30.0,
                    scope: TaskWarriorUrgencyConfigScope::User { property: TaskWarriorUserScopeProperty::Tag },
                },
                urg_n100: TaskWarriorUrgencyConfig {
                    coefficient: -100.0,
                    scope: TaskWarriorUrgencyConfigScope::User { property: TaskWarriorUserScopeProperty::Tag },
                },
            }
        }
    }

    pub fn parse_ini<T>(config_file_name: &str) -> Result<T, FypmError> {
        let config_file_path = Path::new(&CONFIG_PATH.to_string()).join(config_file_name);

        let file_content = fs::read_to_string(&config_file_path).unwrap();

        todo!();
    }
}
