pub mod init_day;
pub mod task;
pub mod timew;
pub mod worktime;
pub mod instance;


use diesel::SqliteConnection;
use diesel::Connection;

use crate::api;
use crate::handlers::aliases;
use crate::handlers::filters::FiltersHandler;
use crate::handlers::worktime::WorktimeHandler;
use crate::{func, DATABASE_URL};

use fypm_lib::values::err::FypmError;

use clap::{Subcommand, ValueEnum, Parser};
use clap_complete::ArgValueCompleter;

use crate::func::completion;

#[derive(Parser)]
#[command(name = "fypm")]
#[command(version = "0.2.0")]
#[command(about = "Four Years Productivity Manager", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Debug, ValueEnum, Clone, PartialEq)]
pub enum VerifyScripts {
    /// Verify if exists Continuous tasks without aliases
    Aliases,
}

#[derive(Debug, ValueEnum, Clone, PartialEq, strum_macros::Display)]
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
#[derive(Debug, ValueEnum, Clone, PartialEq)]
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
    /// Abandon a task in Chain case (alias: h)
    #[value(alias = "h")]
    Chain,
    /// Abandon a task in NoControl case (alias: n)
    #[value(alias = "n")]
    NoControl,
}
#[derive(Debug, ValueEnum, Clone, PartialEq, strum_macros::Display)]
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum StatisticsCommands {
    Deleted,
    Pending,
}
pub enum TimewAction {
    Start,
    End,
}

#[derive(Debug, ValueEnum, Clone, PartialEq)]
pub enum AliasActions {
    Add,
    Change,
}

#[derive(Debug, ValueEnum, Clone, PartialEq)]
pub enum FilterActions {
    Add,
    List,
    Remove,
    Edit,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum Commands {
    //#region               Misc
    /// Generate completions in the current directory
    Completion,
    //#endregion
    //#region               Systems
    /// Start API daemon
    Daemon,

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
    /// Show task information
    TaInfo {
        /// Filter to the task (max: 1)
        filter: String
    },

    /// Add a task to taskwarrior (taadd)
    TaAdd {
        description: String,
        #[arg(add = ArgValueCompleter::new(completion::project))]
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
        /// If you want to specify the annotation for selected
        /// tasks only, you can use this flag to specify the
        /// filter to aplly to the task annotation.
        annotation_filter: Option<String>,
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

    /// Change a recurring task's time (tarecur-t)
    TaRecurTime {
        filter: String,
        new_time: String,
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
        /// Dates to track. You must enter a even number of dates, and all will be tracked in pairs.
        /// Ex: tir 1 10:00 12:00 18:00 18:10
        args: Vec<String>,
    },
    /// Quickly replace a log with just ids (tirep)
    TiReplace {
        original_id: String,
        replacement_id: String,
    },
    /// List logs for a day (tils)
    TiLs {
        #[arg(default_value_t = String::from("today"))]
        initial_date: String,
        final_date: Option<String>,
    },
    //#endregion
}

pub fn matching(command: &Commands) -> Result<(), FypmError> {
    match command {
        //#region               Misc
        Commands::Completion => func::completion::generate_completion(),
        //#endregion
        //#region               Systems
        Commands::Daemon => {
            api::index::rocket().unwrap();

            Ok(())
        },
        Commands::InitDay => todo!(),

        Commands::Verify { script } => func::matchs::match_verify_script(script),

        Commands::Alias { action, filter } => {
            match action {
                AliasActions::Add => aliases::AliasesHandler::add(filter),
                AliasActions::Change => {
                    //aliases::AliasesHandler::change(filter)
                    todo!()
                }
            }
        }

        Commands::Filter { action } => {
            let conn = &mut SqliteConnection::establish(DATABASE_URL.as_str()).unwrap();

            match action {
                FilterActions::Add => FiltersHandler::add(conn),
                FilterActions::List => FiltersHandler::list(conn),
                FilterActions::Remove => FiltersHandler::remove(conn),
                FilterActions::Edit => FiltersHandler::edit(conn),
            }
        },

        Commands::WtAdd { worktime_name } => {
            WorktimeHandler::add(
                &mut SqliteConnection::establish(DATABASE_URL.as_str()).unwrap(),
                worktime_name,
            )?;

            Ok(())
        }
        Commands::WtRemove { worktime_name } => {
            WorktimeHandler::remove(
                &mut SqliteConnection::establish(DATABASE_URL.as_str()).unwrap(),
                worktime_name,
            )?;

            Ok(())
        }
        Commands::WtLs => {
            WorktimeHandler::list(
                &mut SqliteConnection::establish(DATABASE_URL.as_str()).unwrap(),
            )?;

            Ok(())
        }
        Commands::WtApply { worktime_name } => {
            worktime::apply(worktime_name)?;

            Ok(())
        }

        Commands::Instance { action, actionargs } => instance::match_action(action, actionargs),
        //#endregion
        //#region               Task Subcommands
        Commands::TaInfo {
            filter,
        } => task::list::info(filter),

        Commands::TaAdd {
            description,
            project,
            style,
            r#type,
            other_args,
            skip_confirmation,
        } => {
            let execute = task::add::new(
                description,
                project,
                style,
                r#type,
                other_args,
                skip_confirmation,
            );

            match execute.unwrap() {
                _ => Ok(()),
            }
        }
        Commands::TaAddSub {
            mother_task,
            other_args,
            skip_confirmation,
        } => {
            task::add::subtask(mother_task, other_args, skip_confirmation)?;

            Ok(())
        }
        Commands::TaAddSeq {
            seq_type,
            style,
            description,
            project,
            tag,
            initial_number,
            last_number,
            season,
            last_season_id,
        } => task::add::sequence(
            seq_type,
            style,
            description,
            project,
            tag,
            initial_number,
            last_number,
            season,
            last_season_id,
        ),
        Commands::TaAddBrth {
            birthday_person,
            date,
        } => {
            task::add::birthday(birthday_person, date)?;

            Ok(())
        }
        Commands::TaAddPl {
            playlist_name,
            length,
        } => {
            task::add::playlist(playlist_name, length)?;

            Ok(())
        }

        Commands::TaLsDate {
            property,
            modifier,
            date_args,
        } => {
            if let Some(date_args) = date_args {
                task::list::date(property, modifier, date_args)?;
            } else {
                task::list::date(property, modifier, &vec!["-w".to_string()])?;
            }

            Ok(())
        }
        Commands::TaLsMotAndSub { modifier, filter } => {
            task::list::mother_and_subtasks(modifier, filter)?;

            Ok(())
        }
        Commands::TaLsScore { date_args } => {
            if let Some(date_args) = date_args {
                task::list::completion_score(date_args).unwrap();
            } else {
                task::list::completion_score(&vec!["-w".to_string()]).unwrap();
            }

            Ok(())
        }

        Commands::TaStart { filter } => task::update::start(filter),
        Commands::TaStop { filter } => task::update::stop(filter, true),
        Commands::TaDone {
            tasks_to_done,
            tastart_filter,
            annotation,
            skip_confirmation,
            not_necessary,
            delegated,
        } => task::update::done(
            tasks_to_done,
            tastart_filter,
            annotation,
            skip_confirmation,
            not_necessary,
            delegated,
        ),
        Commands::TaAnnotate { filter, annotation } => {
            func::action::annotate("task", filter, annotation, false)
        }
        Commands::TaAbandon {
            tag,
            filter,
            annotation,
            annotation_filter,
        } => task::update::abandon(tag, filter, annotation, annotation_filter),
        Commands::TaStatistic { name, no_parents } => task::list::statistic(name, no_parents),
        Commands::TaSchedule {
            filter,
            alarm_date,
            due_date,
            worktime,
        } => task::update::schedule(filter, alarm_date, due_date, worktime),
        Commands::TaUnschedule {
            filter,
            no_alarm,
            no_due,
            no_worktime,
        } => task::update::unschedule(filter, no_alarm, no_due, no_worktime),
        Commands::TaUnd { filter, unarchive } => task::update::und(filter, unarchive),
        Commands::TaRecurTime { filter, new_time } => {
            task::update::recur_time(filter, new_time)
        }
        Commands::TaProject { action, arg } => task::task_project(action, arg),
        //#endregion
        //#region               Timew Subcommands
        Commands::TiLs { initial_date, final_date } => {
            timew::list(initial_date, final_date)?;

            Ok(())
        }
        Commands::TiEndCorrection {
            manipulation_id,
            reference_id,
        } => timew::move_log(&TimewAction::End, manipulation_id, reference_id),
        Commands::TiStartCorrection {
            manipulation_id,
            reference_id,
        } => timew::move_log(&TimewAction::Start, manipulation_id, reference_id),
        Commands::TiStart { id, start_time } => timew::set_log(&TimewAction::Start, id, start_time),
        Commands::TiEnd { id, end_time } => timew::set_log(&TimewAction::End, id, end_time),
        Commands::TiTrack {
            id,
            args
        } => timew::track(id, args),
        Commands::TiReplace {
            original_id,
            replacement_id,
        } => timew::replace(original_id, replacement_id),
        Commands::TiAnnotate { filter, annotation } => {
            func::action::annotate("timew", filter, annotation, false)
        } //#endregion
    }
}