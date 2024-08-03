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
                }),
                (FypmReports::List, TaskWarriorReportConfig {
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
                }),
                (FypmReports::All, TaskWarriorReportConfig {
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
                }),
                (FypmReports::Blist, TaskWarriorReportConfig {
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
                }),
                (FypmReports::Wlist, TaskWarriorReportConfig {
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
                }),
                (        FypmReports::Goals, TaskWarriorReportConfig {
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
                }),
                (FypmReports::Alarms, TaskWarriorReportConfig {
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
                        "TaskWarriorUserScopeProperty::Tag".to_string(),
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
                }),
                (FypmReports::AllGoals, TaskWarriorReportConfig {
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
                    sort: Some(vec!["GOAL+".to_string()]),
                    filter: Some("status:pending and GOAL.any:".to_string()),
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
                        "TaskWarriorUserScopeProperty::Tag".to_string(),
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

    pub fn parse_ini<T>(config_file_name: &str) -> Result<T, FypmError> {
        let config_file_path = Path::new(&CONFIG_PATH.to_string()).join(config_file_name);

        let file_content = fs::read_to_string(&config_file_path).unwrap();

        todo!();
    }
}
