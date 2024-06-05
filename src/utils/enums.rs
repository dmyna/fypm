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
    /// Anotate on taskwarrior task (tan)
    TaAnnotate { filter: String, annotation: String },
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
    /// Get statistics from taskwarrior
    TaStatistic {
        name: StatisticsCommands,
        /// Exclude recurring tasks from the count
        #[arg(short, long)]
        no_parents: bool,
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

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum StatisticsCommands {
    Deleted,
    Pending,
}
pub enum TimewAction {
    Start,
    End,
}
