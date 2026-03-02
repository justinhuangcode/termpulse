# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- **termpulse-core**: Property-based tests via proptest (sanitize idempotency, write/parse round-trip, strip completeness)
- **termpulse-core**: Criterion benchmarks for all hot paths (write_to, find_sequences, sanitize_label, strip_sequences)
- **termpulse-core**: Fuzz targets for find_sequences, sanitize_label, and strip_sequences (cargo-fuzz / libfuzzer)
- **termpulse-core**: `ParsedSequence::EMPTY` constant for convenient buffer initialization
- **termpulse-core**: Re-exported `is_clean` function (was pub but not accessible from outside the crate)
- **Infrastructure**: SECURITY.md with responsible disclosure policy
- **Infrastructure**: CONTRIBUTING.md with developer onboarding guide
- **Infrastructure**: deny.toml for cargo-deny supply chain auditing (advisories, licenses, bans, sources)
- **Infrastructure**: cargo-deny CI job in GitHub Actions workflow
- **termpulse-core**: `std` feature flag — enables `std::error::Error` impl for `WriteError` (off by default, `no_std` preserved)
- **Packaging**: Per-crate README.md files for crates.io display
- **Packaging**: Added `documentation`, `homepage`, `readme` fields to all crate Cargo.toml files
- **termpulse-core**: `doc_auto_cfg` for automatic feature gate display on docs.rs
- **termpulse-core**: `TryFrom<u8>` and `From<ProgressState> for u8` trait conversions
- **termpulse-core**: `From<WriteError> for std::io::Error` (behind `std` feature)
- **termpulse-core**: `Default` derive on `ProgressState` (Normal) and `Terminator` (St)
- **termpulse-core**: `Hash` derive on `OscSequence`, `ParsedSequence`, and `WriteError`
- **termpulse**: `Copy`, `Hash` on `Multiplexer`; `Copy`, `PartialEq`, `Eq` on `DetectOptions`; `Debug` on `EnvReader`, `Estimator`, `Throttle`, `SilentBackend`
- **Infrastructure**: `no_std` CI verification job (thumbv7m-none-eabi target)
- **Infrastructure**: Select pedantic clippy lints enabled workspace-wide (doc_markdown, missing_errors_doc, missing_panics_doc, and more)
- **termpulse-cli**: `completions` subcommand — generate shell completions for bash, zsh, fish, powershell, elvish
- **Infrastructure**: Feature flag CI testing (`--no-default-features` and `--all-features` for termpulse-core)
- **Infrastructure**: Automated crates.io publishing in release workflow (publish job runs before GitHub Release)
- **Infrastructure**: Justfile with common dev tasks (`check-all`, `test`, `clippy`, `fmt`, `doc`, `test-features`, `check-no-std`, `bench`, `publish-dry-run`, `semver`)
- **Infrastructure**: `cargo-semver-checks` CI job for API compatibility verification
- **Infrastructure**: RELEASING.md with complete publish checklist and version bump instructions
- **termpulse-core**: `#[inline]` hints on hot-path functions (`from_u8`, `as_bytes`, `is_clean`, `has_label`, trait impls)
- **termpulse-cli**: Integration tests for `completions` subcommand (bash, zsh, fish)
- **Packaging**: `authors` and `description` fields in workspace Cargo.toml
- **Packaging**: docs.rs badge added to termpulse-cli README
- **Infrastructure**: Dependabot configuration for Cargo and GitHub Actions dependency updates
- **Infrastructure**: Sleep between crates.io publishes in release workflow (index propagation delay)

### Changed

- **termpulse-core**: Added `#[non_exhaustive]` to all public enums (`ProgressState`, `Terminator`, `WriteError`) and `ParsedSequence` struct for semver safety
- **termpulse**: Added `#[non_exhaustive]` to `TerminalCapability`, `Multiplexer` enums and `DetectOptions` struct
- **termpulse-core**: Added `#[must_use]` to 10+ pure functions to prevent silent result drops
- **termpulse**: Added `#[must_use]` to all detection functions
- **termpulse-core**: Added `#[doc(html_root_url)]` for cross-crate doc linking on docs.rs
- **termpulse**: Added `#[doc(html_root_url)]` for cross-crate doc linking on docs.rs
- All crate Cargo.toml files now use `exclude` to keep published packages lean (no benches, tests, fuzz, or examples)
- Updated .gitignore with fuzz corpus/artifacts, profraw, and proptest-regressions

### Fixed

- **termpulse-core**: `sanitize_label` idempotency bug — `sanitize_label("! ]")` returned `"! "` with trailing space on first call, `"!"` on second call. Fixed by trimming trailing whitespace after truncation.
- **Packaging**: Fixed incorrect `sanitize_label` example in termpulse-core README (output was `"evilinject"`, corrected to `"evil"`)

## [0.1.0] - 2026-02-27

### Added

- **termpulse-core**: `no_std` OSC 9;4 protocol library
  - Sequence construction (`OscSequence`) with all 5 progress states
  - Zero-allocation `write_to()` into caller-provided buffers
  - Full round-trip parser (`find_sequences`) for extracting sequences from text
  - `strip_sequences` for cleaning terminal output
  - `sanitize_label` for preventing OSC escape injection (char-boundary safe)
  - Three terminators: ST, BEL, C1 ST

- **termpulse**: Main library with smart detection and backends
  - Auto-detection of 10+ terminals (Ghostty, WezTerm, iTerm2, Kitty, Windows Terminal, VS Code, ConEmu, Contour, foot, Rio)
  - Three-tier fallback: OSC native -> ASCII progress bar -> Silent
  - tmux DCS passthrough backend for multiplexer environments
  - `NO_COLOR` convention support (https://no-color.org/)
  - Throttle/dedup engine (150ms interval, deduplicates identical updates)
  - ETA estimation via exponential moving average (EMA) algorithm
  - `Controller` API with `auto()`, `set()`, `indeterminate()`, `done()`, `fail()`, `pause()`, `clear()`
  - Full dependency injection via `EnvLookup` and `Backend` traits

- **termpulse-cli**: CLI binary
  - `termpulse set <percent>` — set progress percentage
  - `termpulse start` — indeterminate progress
  - `termpulse done` / `termpulse fail` — signal completion or error
  - `termpulse wrap -- <command>` — wrap any command with progress indication
  - `termpulse pipe` — track piped data progress (bytes or lines)
  - `termpulse clear` — remove progress indicator
  - `termpulse detect` — show terminal capabilities and multiplexer info
  - Global `--json` flag for machine-readable output
  - Ctrl+C signal handling in `wrap` command

- **Infrastructure**
  - Cargo workspace with shared dependencies, lints, and metadata
  - CI pipeline: test (Linux/macOS/Windows), clippy, fmt, doc
  - Integration tests with `assert_cmd`
  - Release profile (LTO, single codegen unit, stripped)

[unreleased]: https://github.com/justinhuangcode/termpulse/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/justinhuangcode/termpulse/releases/tag/v0.1.0
