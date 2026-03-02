//! CLI argument definitions using clap derive API.

use clap::{Parser, Subcommand};
use clap_complete::Shell;

/// termpulse — native terminal progress indicators
///
/// Send OSC 9;4 progress sequences to your terminal with smart detection
/// and graceful fallback. Works with Ghostty, `WezTerm`, iTerm2, Kitty,
/// Windows Terminal, and more.
#[derive(Parser)]
#[command(name = "termpulse", version, about, long_about = None)]
pub struct Cli {
    /// Output in JSON format for machine consumption.
    #[arg(long, global = true)]
    pub json: bool,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Set progress to a specific percentage.
    Set(SetOpts),

    /// Start indeterminate progress (spinner-like).
    Start(StartOpts),

    /// Signal successful completion (emits 100% then clears).
    Done(DoneOpts),

    /// Signal failure (emits error state then clears).
    Fail(FailOpts),

    /// Wrap a command with automatic progress indication.
    ///
    /// Runs the given command and shows progress while it executes.
    /// Progress clears on success, shows error state on failure.
    Wrap(WrapOpts),

    /// Track progress of piped data (bytes or lines).
    ///
    /// Reads stdin and writes to stdout while showing progress.
    Pipe(PipeOpts),

    /// Clear/remove the progress indicator.
    Clear,

    /// Detect terminal capabilities and show support info.
    Detect,

    /// Generate shell completions for the given shell.
    ///
    /// Output the completion script to stdout. Redirect to a file
    /// or source it directly in your shell configuration.
    Completions(CompletionsOpts),
}

#[derive(clap::Args)]
pub struct CompletionsOpts {
    /// The shell to generate completions for.
    pub shell: Shell,
}

#[derive(clap::Args)]
pub struct SetOpts {
    /// Progress percentage (0-100).
    pub percent: u8,

    /// Optional label text.
    #[arg(short, long, default_value = "")]
    pub label: String,
}

#[derive(clap::Args)]
pub struct StartOpts {
    /// Optional label text.
    #[arg(short, long, default_value = "Working")]
    pub label: String,
}

#[derive(clap::Args)]
pub struct DoneOpts {
    /// Optional label text.
    #[arg(short, long, default_value = "Done")]
    pub label: String,
}

#[derive(clap::Args)]
pub struct FailOpts {
    /// Optional label text.
    #[arg(short, long, default_value = "Failed")]
    pub label: String,
}

#[derive(clap::Args)]
pub struct WrapOpts {
    /// The command to run (everything after `--`).
    #[arg(trailing_var_arg = true, required = true)]
    pub command: Vec<String>,

    /// Label shown during execution.
    #[arg(short, long, default_value = "Running")]
    pub label: String,

    /// Label shown on success.
    #[arg(long, default_value = "Done")]
    pub done_label: String,

    /// Label shown on failure.
    #[arg(long, default_value = "Failed")]
    pub fail_label: String,
}

#[derive(clap::Args)]
pub struct PipeOpts {
    /// Total expected bytes (enables percentage progress).
    /// Without this, shows indeterminate progress.
    #[arg(short, long)]
    pub total: Option<u64>,

    /// Count lines instead of bytes.
    #[arg(long)]
    pub lines: bool,

    /// Read buffer size in bytes.
    #[arg(long, default_value = "8192")]
    pub buffer_size: usize,

    /// Label shown during piping.
    #[arg(short, long, default_value = "Piping")]
    pub label: String,
}
