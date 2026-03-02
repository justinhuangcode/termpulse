//! `termpulse set <percent>` command.

use crate::cli::SetOpts;
use anyhow::Result;
use termpulse::Controller;

pub fn run(opts: SetOpts, _json: bool) -> Result<()> {
    let mut ctrl = Controller::auto();
    ctrl.set(opts.percent, &opts.label);
    Ok(())
}
