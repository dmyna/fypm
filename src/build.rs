use clap::CommandFactory;
use clap_complete::{generate_to, Shell};
use std::env;
use std::io::Error;

include!("values/enums.rs");

#[derive(Parser, Debug, PartialEq)]
#[command(name = "completion")]
struct Opt {
    // If provided, outputs the completion file for given shell
    #[arg(long = "generate", value_enum)]
    generator: Option<Shell>,
    #[command(subcommand)]
    command: Option<Commands>,
}

fn main() -> Result<(), Error> {
    let outdir = match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => outdir,
    };

    let mut cmd = Cli::command();

    for &shell in Shell::value_variants() {
        let path = generate_to(shell, &mut cmd, "fypm", outdir.clone())?;

        println!("cargo:warning=completion file is generated: {path:?}");
    }

    Ok(())
}
