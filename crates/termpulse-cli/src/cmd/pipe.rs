//! `termpulse pipe` — read stdin, write stdout, show progress.
//!
//! When `--total` is provided, shows percentage progress.
//! Otherwise, shows indeterminate progress with a byte/line counter.
//! In `--lines` mode, counts newlines instead of bytes.

use crate::cli::PipeOpts;
use anyhow::Result;
use std::io::{self, Read, Write};
use termpulse::Controller;

pub fn run(opts: PipeOpts, json: bool) -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut reader = stdin.lock();
    let mut writer = stdout.lock();
    let mut ctrl = Controller::auto();

    let mut buf = vec![0u8; opts.buffer_size];
    let mut total_bytes: u64 = 0;
    let mut total_lines: u64 = 0;

    // Decide progress mode upfront
    let has_total = opts.total.is_some();
    let expected = opts.total.unwrap_or(0);

    if !has_total {
        ctrl.indeterminate(&opts.label);
    }

    loop {
        let n = reader.read(&mut buf)?;
        if n == 0 {
            break;
        }

        writer.write_all(&buf[..n])?;

        if opts.lines {
            total_lines += buf[..n].iter().filter(|&&b| b == b'\n').count() as u64;
        }
        total_bytes += n as u64;

        // Update progress
        if has_total && expected > 0 {
            let current = if opts.lines { total_lines } else { total_bytes };
            let pct = ((current * 100) / expected).min(100) as u8;
            let label = if opts.lines {
                format!("{} ({}/{})", opts.label, current, expected)
            } else {
                format!("{} ({})", opts.label, format_bytes(current))
            };
            ctrl.set(pct, &label);
        } else {
            // Indeterminate — update counter label
            let label = if opts.lines {
                format!("{} ({} lines)", opts.label, total_lines)
            } else {
                format!("{} ({})", opts.label, format_bytes(total_bytes))
            };
            ctrl.indeterminate(&label);
        }
    }

    writer.flush()?;
    ctrl.done("Done");

    if json {
        let result = serde_json::json!({
            "status": "success",
            "bytes": total_bytes,
            "lines": total_lines,
        });
        eprintln!("{result}");
    }

    Ok(())
}

/// Format bytes into a human-readable string.
fn format_bytes(bytes: u64) -> String {
    const KIB: u64 = 1024;
    const MIB: u64 = 1024 * 1024;
    const GIB: u64 = 1024 * 1024 * 1024;

    if bytes >= GIB {
        format!("{:.1} GiB", bytes as f64 / GIB as f64)
    } else if bytes >= MIB {
        format!("{:.1} MiB", bytes as f64 / MIB as f64)
    } else if bytes >= KIB {
        format!("{:.1} KiB", bytes as f64 / KIB as f64)
    } else {
        format!("{bytes} B")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_bytes_units() {
        assert_eq!(format_bytes(0), "0 B");
        assert_eq!(format_bytes(512), "512 B");
        assert_eq!(format_bytes(1024), "1.0 KiB");
        assert_eq!(format_bytes(1536), "1.5 KiB");
        assert_eq!(format_bytes(1048576), "1.0 MiB");
        assert_eq!(format_bytes(1073741824), "1.0 GiB");
    }
}
