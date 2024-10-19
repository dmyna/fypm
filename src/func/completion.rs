use clap::CommandFactory;
use clap::ValueEnum;
use clap_complete::{aot, CompletionCandidate};
use crate::values::{self, err::FypmError};

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
