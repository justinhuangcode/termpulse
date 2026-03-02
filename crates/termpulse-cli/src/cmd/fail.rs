//! `termpulse fail` command.

use crate::cli::FailOpts;
use anyhow::Result;
use termpulse::Controller;

pub fn run(opts: FailOpts, _json: bool) -> Result<()> {
    let mut ctrl = Controller::auto();
    ctrl.fail(&opts.label);
    Ok(())
}
