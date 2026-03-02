//! `termpulse detect` command.

use anyhow::Result;
use serde::Serialize;
use termpulse::detect::{
    self, DetectOptions, EnvReader, Multiplexer, TerminalCapability, detect_multiplexer,
    multiplexer_supports_passthrough,
};

#[derive(Serialize)]
struct DetectResult {
    capability: String,
    backend: String,
    osc_supported: bool,
    terminal: Option<String>,
    multiplexer: String,
    passthrough: bool,
}

impl std::fmt::Display for DetectResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Terminal capability: {}", self.capability)?;
        writeln!(f, "Backend:            {}", self.backend)?;
        writeln!(f, "OSC 9;4 supported:  {}", self.osc_supported)?;
        if let Some(ref term) = self.terminal {
            writeln!(f, "Terminal:           {term}")?;
        }
        writeln!(f, "Multiplexer:        {}", self.multiplexer)?;
        if self.multiplexer != "none" {
            writeln!(f, "DCS passthrough:    {}", self.passthrough)?;
        }
        Ok(())
    }
}

pub fn run(json: bool) -> Result<()> {
    let cap = detect::detect(&DetectOptions::default());
    let mux = detect_multiplexer(&EnvReader::REAL);
    let passthrough = multiplexer_supports_passthrough(&mux);

    let terminal = std::env::var("TERM_PROGRAM").ok().or_else(|| {
        if std::env::var("WT_SESSION").is_ok() {
            Some("Windows Terminal".to_string())
        } else if std::env::var("ConEmuPID").is_ok() {
            Some("ConEmu".to_string())
        } else {
            std::env::var("TERM").ok()
        }
    });

    let mux_name = match mux {
        Multiplexer::Tmux => "tmux",
        Multiplexer::Screen => "screen",
        Multiplexer::None => "none",
        _ => "unknown",
    };

    // When inside tmux with OSC support, the backend is osc-tmux
    let backend = match cap {
        TerminalCapability::OscProgress if mux == Multiplexer::Tmux => "osc-tmux",
        TerminalCapability::OscProgress => "osc",
        TerminalCapability::AsciFallback => "ascii",
        TerminalCapability::Silent => "silent",
        _ => "unknown",
    };

    let result = DetectResult {
        capability: match cap {
            TerminalCapability::OscProgress => "osc-progress".to_string(),
            TerminalCapability::AsciFallback => "ascii-fallback".to_string(),
            TerminalCapability::Silent => "silent".to_string(),
            _ => "unknown".to_string(),
        },
        backend: backend.to_string(),
        osc_supported: cap == TerminalCapability::OscProgress,
        terminal,
        multiplexer: mux_name.to_string(),
        passthrough,
    };

    if json {
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        print!("{result}");
    }

    Ok(())
}
