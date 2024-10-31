use clap::CommandFactory;
use clap::ValueEnum;
use clap_complete::{aot, CompletionCandidate};
use std::process::Command;

use crate::values::{self, err::FypmError};

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

pub fn generate_completion() -> Result<(), FypmError> {
    let outdir = match std::env::var_os("OUT_DIR") {
        None => std::env::current_dir().unwrap(),
        Some(outdir) => outdir.into(),
    };

    for shell in aot::Shell::value_variants() {
        aot::generate_to(
            shell.clone(),
            &mut values::enums::Cli::command(),
            "fypm",
            &outdir,
        )
        .unwrap();
    }

    Ok(())
}
