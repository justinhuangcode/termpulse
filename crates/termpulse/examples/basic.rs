//! Basic usage of the termpulse library.
//!
//! Run with: `cargo run -p termpulse --example basic`

use termpulse::Controller;

fn main() {
    let mut ctrl = Controller::auto();

    // Simulate a multi-step build process
    ctrl.set(0, "Preparing");
    std::thread::sleep(std::time::Duration::from_millis(500));

    ctrl.set(25, "Compiling");
    std::thread::sleep(std::time::Duration::from_millis(500));

    ctrl.set(50, "Linking");
    std::thread::sleep(std::time::Duration::from_millis(500));

    ctrl.set(75, "Optimizing");
    std::thread::sleep(std::time::Duration::from_millis(500));

    ctrl.done("Build complete");
}
