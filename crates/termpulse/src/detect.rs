//! Terminal capability detection.
//!
//! Detects whether the current terminal supports OSC 9;4 progress indicators
//! using environment variable heuristics.

use std::io::IsTerminal;

/// Terminal capability level for progress indicators.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TerminalCapability {
    /// Terminal supports native OSC 9;4 progress indicators.
    OscProgress,
    /// Terminal is a TTY but OSC 9;4 support is unknown — use ASCII fallback.
    AsciFallback,
    /// Not a TTY (pipe, file, etc.) — silent mode.
    Silent,
}

/// Multiplexer context information.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Multiplexer {
    /// Not running inside a multiplexer.
    None,
    /// Running inside tmux (supports DCS passthrough since tmux 3.3+).
    Tmux,
    /// Running inside GNU screen.
    Screen,
}

/// Options for overriding terminal detection.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct DetectOptions {
    /// Force OSC progress support regardless of detection.
    pub force: bool,
    /// Force disable OSC progress (use fallback or silent).
    pub disabled: bool,
    /// Override TTY detection.
    pub is_tty: Option<bool>,
}

/// Detect the terminal's progress indicator capability.
///
/// Detection order:
/// 1. Explicit overrides (`force`, `disabled`)
/// 2. Environment variable `TERMPULSE_FORCE=1` / `TERMPULSE_DISABLE=1`
/// 3. `NO_COLOR` convention — if set, avoids OSC sequences (uses ASCII fallback)
/// 4. TTY check (non-TTY → Silent)
/// 5. Terminal-specific heuristics
///
/// # Supported terminals
///
/// | Terminal | Detection method |
/// |----------|-----------------|
/// | Ghostty | `TERM_PROGRAM=ghostty` |
/// | WezTerm | `TERM_PROGRAM=wezterm` |
/// | Windows Terminal | `WT_SESSION` env var |
/// | iTerm2 | `TERM_PROGRAM=iTerm.app` |
/// | Kitty | `TERM_PROGRAM=kitty` |
/// | ConEmu | `ConEmuPID` env var |
/// | VS Code Terminal | `TERM_PROGRAM=vscode` |
/// | Contour | `TERM_PROGRAM=contour` |
/// | foot | `TERM=foot*` |
/// | Rio | `TERM_PROGRAM=rio` |
#[must_use]
pub fn detect(opts: &DetectOptions) -> TerminalCapability {
    detect_with_env(opts, &EnvReader::REAL)
}

/// Detect capability with an injectable environment reader (for testing).
#[must_use]
pub fn detect_with_env(opts: &DetectOptions, env: &dyn EnvLookup) -> TerminalCapability {
    // 1. Explicit overrides
    if opts.disabled {
        let is_tty = opts
            .is_tty
            .unwrap_or_else(|| std::io::stderr().is_terminal());
        return if is_tty {
            TerminalCapability::AsciFallback
        } else {
            TerminalCapability::Silent
        };
    }
    if opts.force {
        return TerminalCapability::OscProgress;
    }

    // 2. Environment variable overrides
    if env.var("TERMPULSE_FORCE").as_deref() == Some("1") {
        return TerminalCapability::OscProgress;
    }
    if env.var("TERMPULSE_DISABLE").as_deref() == Some("1") {
        let is_tty = opts
            .is_tty
            .unwrap_or_else(|| std::io::stderr().is_terminal());
        return if is_tty {
            TerminalCapability::AsciFallback
        } else {
            TerminalCapability::Silent
        };
    }

    // 3. NO_COLOR convention (https://no-color.org/)
    // When NO_COLOR is set (to any value), respect the user's preference
    // by falling back to ASCII instead of emitting escape sequences.
    if env.var("NO_COLOR").is_some() {
        let is_tty = opts
            .is_tty
            .unwrap_or_else(|| std::io::stderr().is_terminal());
        return if is_tty {
            TerminalCapability::AsciFallback
        } else {
            TerminalCapability::Silent
        };
    }

    // 4. TTY check
    let is_tty = opts
        .is_tty
        .unwrap_or_else(|| std::io::stderr().is_terminal());
    if !is_tty {
        return TerminalCapability::Silent;
    }

    // 5. Terminal-specific heuristics
    let term_program = env.var("TERM_PROGRAM").unwrap_or_default().to_lowercase();

    // Ghostty
    if term_program.contains("ghostty") {
        return TerminalCapability::OscProgress;
    }

    // WezTerm
    if term_program.contains("wezterm") {
        return TerminalCapability::OscProgress;
    }

    // iTerm2
    if term_program.contains("iterm") {
        return TerminalCapability::OscProgress;
    }

    // Kitty
    if term_program.contains("kitty") {
        return TerminalCapability::OscProgress;
    }

    // VS Code integrated terminal
    if term_program.contains("vscode") {
        return TerminalCapability::OscProgress;
    }

    // Contour
    if term_program.contains("contour") {
        return TerminalCapability::OscProgress;
    }

    // Rio
    if term_program.contains("rio") {
        return TerminalCapability::OscProgress;
    }

    // Windows Terminal (WT_SESSION is set)
    if env.var("WT_SESSION").is_some() {
        return TerminalCapability::OscProgress;
    }

    // ConEmu
    if env.var("ConEmuPID").is_some() {
        return TerminalCapability::OscProgress;
    }

    // foot terminal
    let term = env.var("TERM").unwrap_or_default();
    if term.starts_with("foot") {
        return TerminalCapability::OscProgress;
    }

    // Unknown terminal but is a TTY — ASCII fallback
    TerminalCapability::AsciFallback
}

/// Detect if we're running inside a terminal multiplexer.
///
/// Returns [`Multiplexer::Tmux`] if `TMUX` env var is set, [`Multiplexer::Screen`]
/// if `STY` env var is set (GNU screen), or [`Multiplexer::None`] otherwise.
#[must_use]
pub fn detect_multiplexer(env: &dyn EnvLookup) -> Multiplexer {
    if env.var("TMUX").is_some() {
        Multiplexer::Tmux
    } else if env.var("STY").is_some() {
        Multiplexer::Screen
    } else {
        Multiplexer::None
    }
}

/// Check if the multiplexer context likely supports DCS passthrough for OSC sequences.
///
/// - **tmux**: Supports DCS passthrough since tmux 3.3+ with `allow-passthrough on`.
///   Earlier tmux versions need `set -g allow-passthrough on` in their config.
/// - **screen**: Limited DCS passthrough support. Screen historically supports DCS
///   wrapping but the behavior varies.
///
/// When inside a multiplexer, the outer terminal's capability also matters.
/// This function is intentionally conservative and returns `true` only for tmux,
/// where modern versions handle passthrough well.
#[must_use]
pub fn multiplexer_supports_passthrough(mux: &Multiplexer) -> bool {
    match mux {
        Multiplexer::Tmux => true,    // tmux 3.3+ supports passthrough
        Multiplexer::Screen => false, // Too unreliable across screen versions
        Multiplexer::None => true,    // No multiplexer — passthrough not needed
    }
}

/// Trait for environment variable lookup (dependency injection for testing).
pub trait EnvLookup {
    /// Look up an environment variable by name.
    fn var(&self, name: &str) -> Option<String>;
}

/// Real environment reader.
#[derive(Debug, Clone, Copy)]
pub struct EnvReader;

impl EnvReader {
    /// Singleton for the real process environment.
    pub const REAL: Self = Self;
}

impl EnvLookup for EnvReader {
    fn var(&self, name: &str) -> Option<String> {
        std::env::var(name).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    struct MockEnv(HashMap<String, String>);

    impl MockEnv {
        fn new() -> Self {
            Self(HashMap::new())
        }

        fn set(mut self, key: &str, val: &str) -> Self {
            self.0.insert(key.to_string(), val.to_string());
            self
        }
    }

    impl EnvLookup for MockEnv {
        fn var(&self, name: &str) -> Option<String> {
            self.0.get(name).cloned()
        }
    }

    fn opts_tty() -> DetectOptions {
        DetectOptions {
            is_tty: Some(true),
            ..Default::default()
        }
    }

    fn opts_no_tty() -> DetectOptions {
        DetectOptions {
            is_tty: Some(false),
            ..Default::default()
        }
    }

    #[test]
    fn force_override() {
        let opts = DetectOptions {
            force: true,
            is_tty: Some(false),
            ..Default::default()
        };
        assert_eq!(
            detect_with_env(&opts, &MockEnv::new()),
            TerminalCapability::OscProgress
        );
    }

    #[test]
    fn disabled_override_tty() {
        let opts = DetectOptions {
            disabled: true,
            is_tty: Some(true),
            ..Default::default()
        };
        assert_eq!(
            detect_with_env(&opts, &MockEnv::new()),
            TerminalCapability::AsciFallback
        );
    }

    #[test]
    fn disabled_override_no_tty() {
        let opts = DetectOptions {
            disabled: true,
            is_tty: Some(false),
            ..Default::default()
        };
        assert_eq!(
            detect_with_env(&opts, &MockEnv::new()),
            TerminalCapability::Silent
        );
    }

    #[test]
    fn env_force() {
        let env = MockEnv::new().set("TERMPULSE_FORCE", "1");
        assert_eq!(
            detect_with_env(&opts_tty(), &env),
            TerminalCapability::OscProgress
        );
    }

    #[test]
    fn env_disable() {
        let env = MockEnv::new().set("TERMPULSE_DISABLE", "1");
        assert_eq!(
            detect_with_env(&opts_tty(), &env),
            TerminalCapability::AsciFallback
        );
    }

    #[test]
    fn no_color_falls_back_to_ascii() {
        let env = MockEnv::new()
            .set("NO_COLOR", "1")
            .set("TERM_PROGRAM", "ghostty");
        assert_eq!(
            detect_with_env(&opts_tty(), &env),
            TerminalCapability::AsciFallback
        );
    }

    #[test]
    fn no_color_empty_value_still_triggers() {
        let env = MockEnv::new().set("NO_COLOR", "");
        assert_eq!(
            detect_with_env(&opts_tty(), &env),
            TerminalCapability::AsciFallback
        );
    }

    #[test]
    fn force_overrides_no_color() {
        let env = MockEnv::new()
            .set("NO_COLOR", "1")
            .set("TERMPULSE_FORCE", "1");
        assert_eq!(
            detect_with_env(&opts_tty(), &env),
            TerminalCapability::OscProgress
        );
    }

    #[test]
    fn not_tty_is_silent() {
        assert_eq!(
            detect_with_env(&opts_no_tty(), &MockEnv::new()),
            TerminalCapability::Silent
        );
    }

    #[test]
    fn ghostty_detected() {
        let env = MockEnv::new().set("TERM_PROGRAM", "ghostty");
        assert_eq!(
            detect_with_env(&opts_tty(), &env),
            TerminalCapability::OscProgress
        );
    }

    #[test]
    fn wezterm_detected() {
        let env = MockEnv::new().set("TERM_PROGRAM", "WezTerm");
        assert_eq!(
            detect_with_env(&opts_tty(), &env),
            TerminalCapability::OscProgress
        );
    }

    #[test]
    fn windows_terminal_detected() {
        let env = MockEnv::new().set("WT_SESSION", "some-guid");
        assert_eq!(
            detect_with_env(&opts_tty(), &env),
            TerminalCapability::OscProgress
        );
    }

    #[test]
    fn iterm2_detected() {
        let env = MockEnv::new().set("TERM_PROGRAM", "iTerm.app");
        assert_eq!(
            detect_with_env(&opts_tty(), &env),
            TerminalCapability::OscProgress
        );
    }

    #[test]
    fn kitty_detected() {
        let env = MockEnv::new().set("TERM_PROGRAM", "kitty");
        assert_eq!(
            detect_with_env(&opts_tty(), &env),
            TerminalCapability::OscProgress
        );
    }

    #[test]
    fn conemu_detected() {
        let env = MockEnv::new().set("ConEmuPID", "1234");
        assert_eq!(
            detect_with_env(&opts_tty(), &env),
            TerminalCapability::OscProgress
        );
    }

    #[test]
    fn foot_detected() {
        let env = MockEnv::new().set("TERM", "foot-extra");
        assert_eq!(
            detect_with_env(&opts_tty(), &env),
            TerminalCapability::OscProgress
        );
    }

    #[test]
    fn unknown_tty_is_ascii_fallback() {
        let env = MockEnv::new().set("TERM_PROGRAM", "some-unknown-terminal");
        assert_eq!(
            detect_with_env(&opts_tty(), &env),
            TerminalCapability::AsciFallback
        );
    }

    #[test]
    fn detect_tmux() {
        let env = MockEnv::new().set("TMUX", "/tmp/tmux-1000/default,1234,0");
        assert_eq!(detect_multiplexer(&env), Multiplexer::Tmux);
    }

    #[test]
    fn detect_screen() {
        let env = MockEnv::new().set("STY", "1234.pts-0.hostname");
        assert_eq!(detect_multiplexer(&env), Multiplexer::Screen);
    }

    #[test]
    fn detect_no_multiplexer() {
        assert_eq!(detect_multiplexer(&MockEnv::new()), Multiplexer::None);
    }

    #[test]
    fn tmux_passthrough_supported() {
        assert!(multiplexer_supports_passthrough(&Multiplexer::Tmux));
        assert!(multiplexer_supports_passthrough(&Multiplexer::None));
        assert!(!multiplexer_supports_passthrough(&Multiplexer::Screen));
    }
}
