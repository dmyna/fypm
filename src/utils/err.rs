#[derive(Debug, PartialEq, Eq)]
pub enum FypmErrorKind {
    Aborted,
    AlreadyExists,
    TooMuchTasks,
    /// It occours when a TYPE has a wrong value (ex: Continuous tasks without aliases)
    TaskTypeError,
    NoTasksFound,
    NotEnoughTasks,
    NotFound,
    WrongInitialization,
    ProblemWithStoredTask,
    InvalidInput,
    InvalidConfig,
}

#[derive(Debug)]
pub struct FypmError {
    pub message: String,
    pub kind: FypmErrorKind,
}