//! `termpulse wrap -- <command>` — run a command with progress indication.
//!
//! Shows indeterminate progress while the child process runs. On exit,
//! emits done (exit 0) or fail (non-zero), then exits with the child's code.
//!
//! Signal handling: the child process is spawned in the same process group,
//! so it inherits signals (SIGINT, SIGTERM) from the terminal. We install
//! a Ctrl+C handler to always clear the progress indicator before exiting.

use crate::cli::WrapOpts;
use anyhow::{Context, Result};
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use termpulse::Controller;

pub fn run(opts: WrapOpts, json: bool) -> Result<i32> {
    if opts.command.is_empty() {
        anyhow::bail!("no command provided");
    }

    let program = &opts.command[0];
    let args = &opts.command[1..];

    let mut ctrl = Controller::auto();
    ctrl.indeterminate(&opts.label);

    // Spawn instead of .status() so we can register a ctrlc handler
    // that tracks signal delivery for clean progress cleanup.
    let mut child = Command::new(program)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .with_context(|| format!("failed to execute: {program}"))?;

    // Install Ctrl+C handler. The child also receives SIGINT since it
    // shares the terminal's process group — we just need to know it
    // happened so we can clean up the progress indicator.
    let ctrl_c_fired = Arc::new(AtomicBool::new(false));
    {
        let flag = ctrl_c_fired.clone();
        let _ = ctrlc::set_handler(move || {
            flag.store(true, Ordering::Relaxed);
        });
    }

    let status = child.wait().context("failed to wait on child process")?;
    let exit_code = status.code().unwrap_or(1);

    // Always clear progress indicator, even on signal
    if ctrl_c_fired.load(Ordering::Relaxed) || !status.success() {
        ctrl.fail(&opts.fail_label);
        if json {
            println!(
                "{}",
                serde_json::json!({
                    "status": "error",
                    "exit_code": exit_code,
                    "command": opts.command,
                    "signal": ctrl_c_fired.load(Ordering::Relaxed),
                })
            );
        }
    } else {
        ctrl.done(&opts.done_label);
        if json {
            println!(
                "{}",
                serde_json::json!({
                    "status": "success",
                    "exit_code": 0,
                    "command": opts.command,
                })
            );
        }
    }

    Ok(exit_code)
}
