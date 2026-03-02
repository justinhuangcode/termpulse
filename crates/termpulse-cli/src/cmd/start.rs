//! `termpulse start` command.

use crate::cli::StartOpts;
use anyhow::Result;
use termpulse::Controller;

pub fn run(opts: StartOpts, _json: bool) -> Result<()> {
    let mut ctrl = Controller::auto();
    ctrl.indeterminate(&opts.label);
    Ok(())
}
