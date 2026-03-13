use std::io;

use clap::{Args, CommandFactory};
use clap_complete::{generate, Shell};

use crate::Cli;

#[derive(Args)]
pub struct CompletionsArgs {
    /// Shell to generate completions for
    pub shell: Shell,
}

pub fn run(args: CompletionsArgs) -> anyhow::Result<()> {
    let mut cmd = Cli::command();
    generate(args.shell, &mut cmd, "ezpdf", &mut io::stdout());
    Ok(())
}
