use clap::{Subcommand, ValueEnum};

#[derive(Subcommand)]
pub enum Commands {
    /// Manage daemon processes
    Daemon {
        action: String,
        #[arg(long)]
        name: String,
    },
    /// Manage worktime system
    Worktime {
        action: String,
        #[arg(long)]
        actionargs: Vec<String>,
    },
    /// Manage instances
    Instance {
        action: String,
        #[arg(long)]
        actionargs: Vec<String>,
    },
    /// Initialize day by setting first tasks of the day
    InitDay,

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
        /// The args to be passed to taadd (description and STYLE or simply more than 1 parameter)
        /// or the existent subtask (1 parameter)
        other_args: Vec<String>,
        #[arg(short = 'y', long)]
        skip_confirmation: bool,
    },
    /// Add a sequence of tasks (taadd-seq)
    TaAddSeq {
        seq_type: String,
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
        annotation: Option<String>
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

    /// List logs for a day (tils)
    TiLs {
        #[arg(default_value_t = String::from("today"))]
        date: String,
        filters: Option<Vec<String>>
    },

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
    NoControl
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
