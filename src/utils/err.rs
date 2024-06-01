use std::io::Error;

#[derive(Debug)]
pub enum FypmErrorKind {
    TooMuchTasks,
    NoTasksFound,
    WrongInitialization,
    ProblemWithStoredTask,
    InvalidInput,
}

#[derive(Debug)]
pub struct FypmError {
    pub message: String,
    pub kind: FypmErrorKind
}

pub enum SomeErr {
    Internal(FypmError),
    System(Error)
}