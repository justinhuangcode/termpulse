#![allow(clippy::print_stdout, clippy::print_stderr)]
// All CLI command functions return Result<()> for uniform dispatch in main(),
// even when the current implementation cannot fail.
#![allow(clippy::unnecessary_wraps)]

use clap::Parser;
use std::process::ExitCode;

mod cli;
mod cmd;

fn main() -> ExitCode {
    let args = cli::Cli::parse();

    // Completions is handled separately — writes to stdout and exits.
    if let cli::Command::Completions(opts) = args.command {
        cmd::completions::run(opts);
        return ExitCode::SUCCESS;
    }

    // Wrap is special: it forwards the child's exit code.
    if let cli::Command::Wrap(opts) = args.command {
        return match cmd::wrap::run(opts, args.json) {
            Ok(code) => ExitCode::from(code as u8),
            Err(e) => {
                print_error(&e, args.json);
                ExitCode::FAILURE
            }
        };
    }

    let result = match args.command {
        cli::Command::Set(opts) => cmd::set::run(opts, args.json),
        cli::Command::Start(opts) => cmd::start::run(opts, args.json),
        cli::Command::Done(opts) => cmd::done::run(opts, args.json),
        cli::Command::Fail(opts) => cmd::fail::run(opts, args.json),
        cli::Command::Pipe(opts) => cmd::pipe::run(opts, args.json),
        cli::Command::Clear => cmd::clear::run(args.json),
        cli::Command::Detect => cmd::detect::run(args.json),
        cli::Command::Wrap(_) | cli::Command::Completions(_) => unreachable!(),
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            print_error(&e, args.json);
            ExitCode::FAILURE
        }
    }
}

fn print_error(e: &anyhow::Error, json: bool) {
    if json {
        let msg = serde_json::json!({
            "error": e.to_string(),
        });
        eprintln!("{msg}");
    } else {
        eprintln!("error: {e}");
    }
}
