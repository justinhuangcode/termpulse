//! `termpulse done` command.

use crate::cli::DoneOpts;
use anyhow::Result;
use termpulse::Controller;

pub fn run(opts: DoneOpts, _json: bool) -> Result<()> {
    let mut ctrl = Controller::auto();
    ctrl.done(&opts.label);
    Ok(())
}
