use clap::{CommandFactory, Parser};
use clap_complete::{generate, Shell};
use miette::Result;

#[derive(Debug, Parser)]
pub struct Completions {
    /// The shell syntax to use. Infers when missing.
    #[clap(long, value_enum)]
    shell: Option<Shell>,
}

pub fn run(opts: &Completions) -> Result<()> {
    let shell = opts
        .shell
        .or_else(|| Shell::from_env())
        .ok_or_else(|| miette::miette!("failed to infer shell"))?;

    let mut cmd = crate::Cli::command();

    log::info!("Generating completions for {shell:?}...");

    generate(shell, &mut cmd, "tauri-bindgen", &mut std::io::stdout());

    Ok(())
}
