//! Integration tests for the `termpulse` CLI binary.
//!
//! Uses `assert_cmd` to test the actual compiled binary.

#![allow(deprecated)] // cargo_bin deprecation — macro alternative not yet stable

use assert_cmd::Command;
use predicates::prelude::*;

fn cmd() -> Command {
    Command::cargo_bin("termpulse").unwrap()
}

// -- help / version --

#[test]
fn help_shows_usage() {
    cmd()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("termpulse"));
}

#[test]
fn version_shows_version() {
    cmd()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("termpulse"));
}

// -- detect --

#[test]
fn detect_runs_successfully() {
    cmd().arg("detect").assert().success();
}

#[test]
fn detect_json_outputs_valid_json() {
    let output = cmd()
        .args(["detect", "--json"])
        .output()
        .expect("failed to run");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&stdout).expect("invalid JSON");
    assert!(parsed.get("capability").is_some());
    assert!(parsed.get("backend").is_some());
    assert!(parsed.get("osc_supported").is_some());
    assert!(parsed.get("multiplexer").is_some());
}

// -- set --

#[test]
fn set_accepts_percent() {
    // In a non-TTY (piped) context, this goes to silent backend — no output expected
    cmd().args(["set", "50"]).assert().success();
}

#[test]
fn set_with_label() {
    cmd()
        .args(["set", "75", "-l", "Building"])
        .assert()
        .success();
}

// -- start / done / fail / clear --

#[test]
fn start_runs() {
    cmd().arg("start").assert().success();
}

#[test]
fn done_runs() {
    cmd().arg("done").assert().success();
}

#[test]
fn fail_runs() {
    cmd().arg("fail").assert().success();
}

#[test]
fn clear_runs() {
    cmd().arg("clear").assert().success();
}

// -- wrap --

#[test]
fn wrap_success_exit_zero() {
    cmd().args(["wrap", "--", "true"]).assert().success();
}

#[test]
fn wrap_failure_exit_nonzero() {
    cmd().args(["wrap", "--", "false"]).assert().failure();
}

#[test]
fn wrap_json_output() {
    let output = cmd()
        .args(["--json", "wrap", "--", "true"])
        .output()
        .expect("failed to run");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&stdout).expect("invalid JSON");
    assert_eq!(parsed["status"], "success");
    assert_eq!(parsed["exit_code"], 0);
}

#[test]
fn wrap_no_command_fails() {
    cmd().arg("wrap").assert().failure();
}

// -- pipe --

#[test]
fn pipe_reads_stdin() {
    cmd()
        .args(["pipe", "-l", "Test"])
        .write_stdin("hello world\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("hello world"));
}

#[test]
fn pipe_with_total() {
    cmd()
        .args(["pipe", "--total", "11"])
        .write_stdin("hello world")
        .assert()
        .success()
        .stdout("hello world");
}

#[test]
fn pipe_line_mode() {
    cmd()
        .args(["pipe", "--lines", "--total", "2"])
        .write_stdin("line1\nline2\n")
        .assert()
        .success()
        .stdout("line1\nline2\n");
}

// -- completions --

#[test]
fn completions_bash() {
    cmd()
        .args(["completions", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains("termpulse"));
}

#[test]
fn completions_zsh() {
    cmd()
        .args(["completions", "zsh"])
        .assert()
        .success()
        .stdout(predicate::str::contains("termpulse"));
}

#[test]
fn completions_fish() {
    cmd()
        .args(["completions", "fish"])
        .assert()
        .success()
        .stdout(predicate::str::contains("termpulse"));
}
