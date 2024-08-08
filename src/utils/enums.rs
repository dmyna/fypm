use clap::{Subcommand, ValueEnum};
use strum::{Display, EnumString};

#[derive(ValueEnum, Clone, PartialEq)]
pub enum VerifyScripts {
    /// Verify if exists Continuous tasks without aliases
    Aliases,
}

#[derive(ValueEnum, Clone, PartialEq, strum_macros::Display)]
pub enum TaProjectActions {
    /// Add a project (alias: a)
    #[value(alias = "a")]
    Add,
    /// List projects (alias: l)
    #[value(alias = "l")]
    List,
    /// Archive a project (alias: c)
    #[value(alias = "c")]
    Archive,
    /// Unarchive a project (alias: u)
    #[value(alias = "u")]
    Unarchive,
}
#[derive(ValueEnum, Clone, PartialEq)]
pub enum TaAbandonTags {
    /// Archive a task (alias: c)
    #[value(alias = "c")]
    Archived,
    /// Abandon a task in Failed case (alias: f)
    #[value(alias = "f")]
    Failed,
    /// Abandon a task in Abandoned case (alias: a)
    #[value(alias = "a")]
    Abandoned,
    /// Abandon a task in NoControl case (alias: n)
    #[value(alias = "n")]
    NoControl,
}
#[derive(ValueEnum, Clone, PartialEq, strum_macros::Display)]
pub enum TaSequenceTypes {
    /// Create a book sequence
    #[value(alias = "b")]
    Book,
    /// Create a serie sequence
    #[value(alias = "s")]
    Serie,
    /// Create an anime sequence
    #[value(alias = "a")]
    Anime,
    /// Create a Youtube playlist sequence
    #[value(alias = "yp")]
    YoutubePlaylist,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum StatisticsCommands {
    Deleted,
    Pending,
}
pub enum TimewAction {
    Start,
    End,
}

#[derive(ValueEnum, Clone, PartialEq)]
pub enum AliasActions {
    Add,
    Change,
}

#[derive(ValueEnum, Clone, PartialEq)]
pub enum FilterActions {
    Add,
    List,
    Remove,
    Edit,
}

#[derive(Subcommand)]
pub enum Commands {
    //#region               Systems
    /// Add a worktime
    WtAdd { worktime_name: String },
    /// Remove a worktime
    WtRemove { worktime_name: String },
    /// List worktimes
    WtLs,
    /// Apply a worktime
    WtApply { worktime_name: String },

    /// Verify tasks for inconsistencies
    Verify { script: VerifyScripts },

    /// Manage tasks aliases
    Alias {
        /// The action to be performed
        action: AliasActions,
        /// Filter to task to be manipulated (max: 1)
        filter: String,
    },

    /// Manage filters
    Filter {
        /// The action to be performed
        action: FilterActions,
    },

    /// Manage instances
    Instance {
        action: String,
        #[arg(long)]
        actionargs: Vec<String>,
    },
    /// Initialize day by setting first tasks of the day
    InitDay,
    //#endregion
    //#region               Task Subcommands
    /// Add a task to taskwarrior (taadd)
    TaAdd {
        description: String,
        project: String,
        style: String,
        r#type: String,
        other_args: Option<Vec<String>>,
        #[arg(short = 'y', long)]
        skip_confirmation: bool,
    },
    /// Add a subtask to a objective task (taadd-sub)
    TaAddSub {
        mother_task: String,
        /// The args to be passed to taadd (required: description, STYLE, TYPE)
        /// or the existent subtask (1 parameter)
        other_args: Vec<String>,
        #[arg(short = 'y', long)]
        skip_confirmation: bool,
    },
    /// Add a sequence of tasks (taadd-seq)
    TaAddSeq {
        seq_type: TaSequenceTypes,
        style: String,
        description: String,
        project: String,
        /// An uniq [a-Z][0-9]{4} identifier to the sequence
        /// (it's recommended to use a tag that remembers the task).
        tag: String,
        /// The number of the first SubTask
        initial_number: usize,
        /// The number of the last SubTask
        last_number: usize,
        season: Option<String>,
        /// Inform the number of the last sequence task to link
        /// its last subtask with the first subtask of the new sequence
        last_season_id: Option<String>,
    },
    /// Add a birthday event (taadd-brth)
    TaAddBrth {
        birthday_person: String,
        /// Format: MM-DD
        date: String,
    },
    /// Add a playlist task (taadd-pl)
    TaAddPl {
        playlist_name: String,
        /// Quantity of songs that you wish to add to the playlist
        length: u16,
    },
    /// Anotate on taskwarrior task (tan)
    TaAnnotate { filter: String, annotation: String },
    /// Abandon a task (taban)
    TaAbandon {
        #[arg(value_enum)]
        tag: TaAbandonTags,
        filter: String,
        /// Required for 'abandoned' (a) and 'no-control' (n).
        annotation: Option<String>,
    },
    /// Start a task (tastart)
    TaStart { filter: String },
    /// Stop a task (tastop)
    TaStop { filter: Option<String> },
    /// Mark a task as done (tadone)
    TaDone {
        tasks_to_done: Option<String>,
        #[arg(short = 's', long = "start")]
        tastart_filter: Option<String>,
        /// Add an annotation to selected tasks
        #[arg(short = 'a', long = "annotation")]
        annotation: Option<String>,
        /// Skip confirmation
        #[arg(short = 'y', long = "skip")]
        skip_confirmation: bool,
        /// Didn't need to do a task and it's done? Tag it with this tag!
        /// (If you're prevented from doing the task, use `taban n` even if it's no longer needed)
        #[arg(short = 'n', long = "not-necessary")]
        not_necessary: bool,
        /// Have you delegated this task and it was done? Tag it with this tag!
        #[arg(short = 'd', long = "delegated")]
        delegated: bool,
    },
    TaSchedule {
        filter: String,
        alarm_date: String,
        due_date: Option<String>,
        /// Specify a worktime
        #[arg(short = 'w', long = "worktime")]
        worktime: Option<String>,
    },
    TaUnschedule {
        filter: String,
        #[arg(short = 'a', long)]
        no_alarm: bool,
        #[arg(short = 'd', long)]
        no_due: bool,
        #[arg(short = 'w', long)]
        no_worktime: bool,
    },
    /// Set a task as pending, removing the "failed/abandoned/no-control" status or unarchiving it (taund)
    TaUnd {
        filter: String,
        /// Unarchive a task (alias: u)
        #[arg(short = 'u', long = "unarchive")]
        unarchive: bool,
    },
    TaProject {
        #[arg(value_enum)]
        action: TaProjectActions,
        /// Project || Filter. Project is required in "a && c" options. Filter is optional in "l" flag.
        #[arg(short, long)]
        arg: Option<String>,
    },
    /// Get statistics from taskwarrior (tastat-*)
    TaStatistic {
        name: StatisticsCommands,
        /// Exclude recurring tasks from the count
        #[arg(short, long)]
        no_parents: bool,
    },
    /// List tasks by date in a separate day/week style (tals-date)
    TaLsDate {
        property: String,
        modifier: String,
        /// To specify range, use: -- -<y|m|w> [year|month|week] OR <start_date> - <end_date> (where format is YYYY-MM-DD)
        date_args: Option<Vec<String>>,
    },
    /// List tasks with mothers and them subtasks agrouped (tamas)
    TaLsMotAndSub {
        modifier: String,
        filter: Vec<String>,
    },
    /// List score of tasks between failed/abandoned/no-control and completed tasks (tals-score)
    TaLsScore {
        /// To specify range, use: -- -<y|m|w> [year|month|week] OR <start_date> - <end_date> (where format is YYYY-MM-DD)
        date_args: Option<Vec<String>>,
    },
    //#endregion
    //#region               Timew Subcommands
    /// Anotate on timewarrior task (tin)
    TiAnnotate { filter: String, annotation: String },
    /// Move start of a task to end of other (ticart)
    TiStartCorrection {
        #[arg(default_value_t = String::from("@1"))]
        manipulation_id: String,
        reference_id: Option<String>,
    },
    /// Move end of a task to start of other (ticend)
    TiEndCorrection {
        #[arg(default_value_t = String::from("@3"))]
        manipulation_id: String,
        reference_id: Option<String>,
    },
    /// Move start of a task to end of other (tistart)
    TiStart { id: String, start_time: String },
    /// Move start of a task to end of other (tiend)
    TiEnd { id: String, end_time: String },
    /// Track a task manually (tir)
    TiTrack {
        id: String,
        start_time: String,
        end_time: String,
    },
    /// Quickly replace a log with just ids (tirep)
    TiReplace {
        original_id: String,
        replacement_id: String,
    },
    /// List logs for a day (tils)
    TiLs {
        #[arg(default_value_t = String::from("today"))]
        date: String,
        filters: Option<Vec<String>>,
    },
    //#endregion
}

#[derive(EnumString, Display, Hash, PartialEq, Eq, Debug, Ord, PartialOrd)]
#[strum(serialize_all = "kebab-case")]
pub enum FypmReports {
    Waiting,
    Next,
    List,
    All,
    Blist,
    Wlist,
    Goals,
    Alarms,
    AllGoals,
    Const,
    Recurring,
    Visual,
}
#[derive(EnumString, Display, Hash, PartialEq, Eq, Debug, Ord, PartialOrd)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum FypmUDAs {
    Style,
    Type,
    State,
    Mother,
    Inforelat,
    SeqCurrent,
    SeqPrevious,
    SeqNext,
    Wt,
    Goal,
    Alarm,
    #[strum(serialize = "effort")]
    Effort,
    #[strum(serialize = "quadrant")]
    Quadrant,
    #[strum(serialize = "estimate")]
    Estimate,
}
#[derive(EnumString, Display, Hash, PartialEq, Eq, Debug, Ord, PartialOrd)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum FypmUrgency {
    // General
    #[strum(serialize = "active")]
    Active,
    #[strum(serialize = "tags")]
    Tags,
    #[strum(serialize = "project")]
    Project,
    #[strum(serialize = "annotations")]
    Annotations,
    #[strum(serialize = "scheduled")]
    Scheduled,

    // Virtual Tags
    #[strum(serialize = "OVERDUE")]
    Overdue,
    #[strum(serialize = "WAITING")]
    Waiting,
    #[strum(serialize = "TEMPLATE")]
    Template,
    #[strum(serialize = "COMPLETED")]
    Completed,
    #[strum(serialize = "DELETED")]
    Deleted,

    // WorkTime
    #[strum(serialize = "WT-Quantify!")]
    WtQuantify,
    #[strum(serialize = "WT-AllDay!")]
    WtAllDay,
    #[strum(serialize = "WT-NonSched!")]
    WtNonSched,

    // Type
    #[strum(serialize = "TYPE-Eventual")]
    TypeEventual,
    #[strum(serialize = "TYPE-Habit")]
    TypeHabit,
    #[strum(serialize = "TYPE-Objective")]
    TypeObjective,
    #[strum(serialize = "TYPE-Continuous")]
    TypeContinuous,
    #[strum(serialize = "TYPE-Check")]
    TypeCheck,
    #[strum(serialize = "TYPE-Event")]
    TypeEvent,
    #[strum(serialize = "TYPE-Goal")]
    TypeGoal,

    // Style
    #[strum(serialize = "STYLE-Apollonian")]
    StyleApollonian,
    #[strum(serialize = "STYLE-Creative")]
    StyleCreative,
    #[strum(serialize = "STYLE-Dionysian")]
    StyleDionysian,
    #[strum(serialize = "STYLE-Necessity")]
    StyleNecessity,
    #[strum(serialize = "STYLE-Idle")]
    StyleIdle,

    // Effort
    #[strum(serialize = "effort-Zero")]
    EffortZero,
    #[strum(serialize = "effort-One")]
    EffortOne,
    #[strum(serialize = "effort-Two")]
    EffortTwo,
    #[strum(serialize = "effort-Three")]
    EffortThree,
    #[strum(serialize = "effort-Four")]
    EffortFour,
    #[strum(serialize = "effort-Five")]
    EffortFive,

    // Quadrant
    #[strum(serialize = "quadrant-One")]
    QuadrantOne,
    #[strum(serialize = "quadrant-Two")]
    QuadrantTwo,
    #[strum(serialize = "quadrant-Three")]
    QuadrantThree,
    #[strum(serialize = "quadrant-Four")]
    QuadrantNone,

    // Fypm Tags
    #[strum(serialize = "SUBTASK")]
    SubTask,

    // Urgency Increment
    UrgP5,
    UrgP10,
    UrgP15,
    UrgP20,
    UrgP25,
    UrgP30,
    UrgP100,
    UrgN5,
    UrgN10,
    UrgN15,
    UrgN20,
    UrgN25,
    UrgN30,
    UrgN100,
}
