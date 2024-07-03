use std::io::Error;

#[derive(Debug)]
pub enum FypmErrorKind {
    Aborted,
    AlreadyExists,
    TooMuchTasks,
    /// It occours when a TYPE has a wrong value (ex: Continuous tasks without aliases)
    TaskTypeError,
    NoTasksFound,
    NotFound,
    WrongInitialization,
    ProblemWithStoredTask,
    InvalidInput,
}

#[derive(Debug)]
pub struct FypmError {
    pub message: String,
    pub kind: FypmErrorKind,
}

pub enum SomeErr {
    Internal(FypmError),
    System(Error),
}
