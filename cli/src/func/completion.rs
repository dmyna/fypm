use clap::CommandFactory;
use clap::ValueEnum;
use clap_complete::{aot, CompletionCandidate};
use std::process::Command;

use crate::commands;

use fypm_lib::values::err::FypmError;

/// Return a vector of CompletionCandidate for the given current string.
///
/// It gets all projects from taskwarrior and filter them by the given current string.
/// If the current string is empty, it will return an empty vector.
pub fn project(current: &std::ffi::OsStr) -> Vec<CompletionCandidate> {
    let mut completions = vec![];
    let Some(current) = current.to_str() else {
        return completions;
    };

    let get_candidates = Command::new("task")
        .arg("_projects")
        .output()
        .unwrap()
        .stdout;

    let candidates = String::from_utf8(get_candidates)
        .unwrap()
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>();

    println!("{:?}", &candidates);

    for candidate in &candidates {
        if candidate.starts_with(current) {
            completions.push(CompletionCandidate::new(candidate));
        }
    }

    completions
}

/// Generate completion scripts for all shells supported by clap_complete
///
/// If the "OUT_DIR" environment variable is set, the completion scripts will be
/// written to that directory. Otherwise, the current directory is used.
///
/// The generated completion scripts are named after the shell, with the name
/// being the lowercase version of the shell name followed by "-fypm-completion".
///
/// # Errors
///
/// If the generation fails for any reason, an error is returned.
pub fn generate_completion() -> Result<(), FypmError> {
    let outdir = match std::env::var_os("OUT_DIR") {
        None => std::env::current_dir().unwrap(),
        Some(outdir) => outdir.into(),
    };

    for shell in aot::Shell::value_variants() {
        aot::generate_to(
            shell.clone(),
            &mut commands::Cli::command(),
            "fypm",
            &outdir,
        )
        .unwrap();
    }

    Ok(())
}
