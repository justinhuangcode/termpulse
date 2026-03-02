//! `termpulse clear` command.

use anyhow::Result;
use termpulse::Controller;

pub fn run(_json: bool) -> Result<()> {
    let mut ctrl = Controller::auto();
    ctrl.clear();
    Ok(())
}
