//! Shell completions generation.

use crate::cli::{Cli, CompletionsOpts};
use clap::CommandFactory;
use clap_complete::generate;

pub fn run(opts: CompletionsOpts) {
    let mut cmd = Cli::command();
    generate(opts.shell, &mut cmd, "termpulse", &mut std::io::stdout());
}
