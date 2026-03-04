# TermPulse

**English** | [中文](./README_CN.md)

[![CI](https://github.com/justinhuangcode/termpulse/actions/workflows/ci.yml/badge.svg)](https://github.com/justinhuangcode/termpulse/actions/workflows/ci.yml)
[![Release](https://github.com/justinhuangcode/termpulse/actions/workflows/release.yml/badge.svg)](https://github.com/justinhuangcode/termpulse/actions/workflows/release.yml)
[![Crates.io](https://img.shields.io/crates/v/termpulse?style=flat-square)](https://crates.io/crates/termpulse)
[![docs.rs](https://img.shields.io/docsrs/termpulse?style=flat-square&logo=docs.rs&logoColor=white)](https://docs.rs/termpulse)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.85%2B-orange.svg?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org)
[![GitHub Stars](https://img.shields.io/github/stars/justinhuangcode/termpulse?style=flat-square&logo=github)](https://github.com/justinhuangcode/termpulse/stargazers)
[![Last Commit](https://img.shields.io/github/last-commit/justinhuangcode/termpulse?style=flat-square)](https://github.com/justinhuangcode/termpulse/commits/main)
[![Issues](https://img.shields.io/github/issues/justinhuangcode/termpulse?style=flat-square)](https://github.com/justinhuangcode/termpulse/issues)
[![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Linux%20%7C%20Windows-lightgrey?style=flat-square)](https://github.com/justinhuangcode/termpulse)

A native terminal progress indicator CLI for smart detection, graceful fallback, and zero configuration. 📊

## Why TermPulse?

Build tools, CI pipelines, and download scripts need to show progress. But terminal progress is harder than it looks:

- **No standard API** — OSC 9;4 is the closest thing, but every terminal supports it differently
- **Silent failures** — emit the wrong escape sequence and you get garbage in the output
- **No fallback** — if the terminal doesn't support OSC, your progress just vanishes

Existing solutions don't solve this:

| | termpulse | [osc-progress](https://github.com/steipete/osc-progress) | Raw escape codes |
| --- | --- | --- | --- |
| Language | Rust (with CLI) | TypeScript (library only) | Any |
| Terminal detection | 10+ terminals, auto-detect | 3 terminals, manual | None |
| Graceful fallback | OSC > ASCII > Silent | None | None |
| tmux / screen | DCS passthrough | No | Manual wrapping |
| `NO_COLOR` respected | Yes | No | Manual |
| Ctrl+C cleanup | Yes | No | Manual |
| Throttle / dedup | Built-in 150ms | Manual | Manual |
| ETA estimation | EMA algorithm | No | Manual |
| Label injection safety | Sanitized | No | Manual |
| Sequence parser | Full round-trip | No | Manual |
| `no_std` core | Zero deps, WASM-ready | No | N/A |

termpulse detects the terminal, picks the best output method, and falls back gracefully. You call `termpulse set 50` and it just works.

## Features

- **Zero-config detection** — auto-detects Ghostty, WezTerm, iTerm2, Kitty, Windows Terminal, VS Code, ConEmu, Contour, foot, Rio, and more
- **Three-tier fallback** — OSC 9;4 native progress > ASCII progress bar on stderr > silent mode; never crashes, never corrupts output
- **tmux DCS passthrough** — detects `TMUX` environment, wraps OSC sequences in DCS passthrough envelope for tmux 3.3+
- **`wrap` command** — run any shell command with automatic indeterminate progress; signals done/fail on exit; forwards child exit code
- **`pipe` command** — transparent stdin-to-stdout pipe that tracks bytes or lines; shows percentage with `--total` or indeterminate counter without
- **Throttle and dedup engine** — rate-limits backend writes to 150ms intervals; deduplicates identical state; passes state and label changes immediately
- **ETA estimation** — exponential moving average (EMA) algorithm with configurable alpha; human-readable display capped at 24 hours
- **Label sanitization** — strips ESC, BEL, C1 ST, and control characters from labels at zero cost; prevents OSC escape injection
- **Signal handling** — installs `ctrlc` handler in `wrap` mode to always clear the progress indicator before exit, even on Ctrl+C
- **`NO_COLOR` support** — respects the [no-color.org](https://no-color.org/) convention; `TERMPULSE_FORCE` overrides when needed
- **`no_std` core** — `termpulse-core` has zero dependencies, `#![no_std]`, `forbid(unsafe_code)`; works in embedded, WASM, and FFI contexts
- **Dependency injection** — all I/O goes through traits (`Backend`, `EnvLookup`, `Write`); 111 tests with full mock coverage

## Installation

### Pre-built binaries (recommended)

Download from [GitHub Releases](https://github.com/justinhuangcode/termpulse/releases):

| Platform | Archive |
| --- | --- |
| Linux x86_64 | `termpulse-x86_64-unknown-linux-gnu.tar.gz` |
| Linux aarch64 | `termpulse-aarch64-unknown-linux-gnu.tar.gz` |
| macOS x86_64 | `termpulse-x86_64-apple-darwin.tar.gz` |
| macOS Apple Silicon | `termpulse-aarch64-apple-darwin.tar.gz` |
| Windows x86_64 | `termpulse-x86_64-pc-windows-msvc.zip` |

### Via Cargo

```bash
cargo install termpulse-cli
```

### From source

```bash
git clone https://github.com/justinhuangcode/termpulse.git
cd termpulse
cargo install --path crates/termpulse-cli
```

**Requirements:** Rust 1.85+

## Quick Start

### CLI

```bash
# Set progress to 50%
termpulse set 50 -l "Building"

# Indeterminate spinner
termpulse start -l "Compiling"

# Wrap a command — shows progress, forwards exit code
termpulse wrap -- cargo build --release

# Pipe with progress tracking
curl -sL https://example.com/file.tar.gz \
  | termpulse pipe --total 104857600 -l "Downloading" \
  > file.tar.gz

# Signal completion
termpulse done -l "Build complete"
termpulse fail -l "Build failed"

# Detect terminal capabilities
termpulse detect --json
```

### Rust library

```rust
use termpulse::Controller;

let mut ctrl = Controller::auto();
ctrl.set(25, "Downloading");
ctrl.set(50, "Downloading");
ctrl.set(75, "Downloading");
ctrl.done("Complete");
```

### Core (`no_std`)

```rust
use termpulse_core::{OscSequence, ProgressState, Terminator};

let seq = OscSequence::normal_with_label(50, "Building");
let mut buf = [0u8; 256];
let n = seq.write_to(&mut buf).unwrap();
// buf[..n] = b"\x1b]9;4;1;50;Building\x1b\\"
```

## Commands

| Command | Description |
| --- | --- |
| `set <percent> [-l label]` | Set progress percentage (0-100) |
| `start [-l label]` | Start indeterminate progress |
| `done [-l label]` | Signal success (100% then clear) |
| `fail [-l label]` | Signal failure (error state then clear) |
| `wrap -- <command...>` | Wrap a shell command with progress |
| `pipe [--total N] [--lines]` | Pipe stdin to stdout with progress |
| `clear` | Clear/remove the progress indicator |
| `detect` | Show terminal capabilities |
| `completions <shell>` | Generate shell completions (bash, zsh, fish, powershell, elvish) |

### Global flags

| Flag | Description |
| --- | --- |
| `--json` | Output in JSON format |

### `wrap` flags

| Flag | Default | Description |
| --- | --- | --- |
| `-l, --label` | `Running` | Label shown during execution |
| `--done-label` | `Done` | Label shown on success |
| `--fail-label` | `Failed` | Label shown on failure |

### `pipe` flags

| Flag | Default | Description |
| --- | --- | --- |
| `-t, --total` | — | Total expected bytes (enables percentage) |
| `--lines` | `false` | Count lines instead of bytes |
| `--buffer-size` | `8192` | Read buffer size in bytes |
| `-l, --label` | `Piping` | Label shown during piping |

## How It Works

1. `Controller::auto()` reads environment variables (`TERM_PROGRAM`, `WT_SESSION`, `TMUX`, etc.)
2. Detects the terminal and selects the best backend: **OSC 9;4**, **ASCII**, or **Silent**
3. If inside tmux, wraps OSC sequences in DCS passthrough (`\ePtmux;...\e\\`)
4. Throttle engine rate-limits writes to 150ms; deduplicates identical updates
5. Label sanitizer strips dangerous bytes (ESC, BEL, control chars) before embedding
6. On done/fail, emits final state and clears the indicator

```
termpulse set 50 -l "Building"
        |
        v
  detect terminal (env vars)
        |
        v
  select backend (OSC / ASCII / Silent)
        |
        v
  throttle + dedup (150ms, skip identical)
        |
        v
  sanitize label (strip ESC/BEL/control)
        |
        v
  emit to stderr (\x1b]9;4;1;50;Building\x1b\\)
```

## Architecture

```
                    Cargo Workspace
+------------------+    +------------------+    +------------------+
|  termpulse-core  |    |    termpulse     |    |  termpulse-cli   |
|  (no_std, 0 dep) |--->| (library, 1 dep) |--->|  (binary, 5 dep) |
+------------------+    +------------------+    +------------------+
| OscSequence      |    | Controller       |    | set / start      |
| ProgressState    |    | detect()         |    | done / fail      |
| find_sequences() |    | Backend trait    |    | wrap / pipe      |
| sanitize_label() |    |   OscBackend     |    | clear / detect   |
| strip_sequences()|    |   TmuxBackend    |    |                  |
|                  |    |   AsciiBackend   |    |                  |
|                  |    |   SilentBackend  |    |                  |
|                  |    | Throttle         |    |                  |
|                  |    | Estimator        |    |                  |
+------------------+    +------------------+    +------------------+
```

**Core narrow, outer wide** — the inner crate has maximum constraints (`no_std`, zero dependencies, `forbid(unsafe_code)`) while outer crates progressively add capabilities:

| Crate | `no_std` | Dependencies | Purpose |
| --- | --- | --- | --- |
| `termpulse-core` | Yes | 0 | OSC 9;4 build, parse, sanitize, strip |
| `termpulse` | No | 1 (termpulse-core) | Detection, backends, throttle, ETA |
| `termpulse-cli` | No | 5 (anyhow, clap, ctrlc, serde, serde_json) | CLI binary |

## Terminal Support

| Terminal | Detection method | Support |
| --- | --- | --- |
| Ghostty | `TERM_PROGRAM=ghostty` | OSC 9;4 native |
| WezTerm | `TERM_PROGRAM=wezterm` | OSC 9;4 native |
| iTerm2 | `TERM_PROGRAM=iTerm.app` | OSC 9;4 native |
| Kitty | `TERM_PROGRAM=kitty` | OSC 9;4 native |
| Windows Terminal | `WT_SESSION` env var | OSC 9;4 native |
| VS Code Terminal | `TERM_PROGRAM=vscode` | OSC 9;4 native |
| ConEmu | `ConEmuPID` env var | OSC 9;4 native |
| Contour | `TERM_PROGRAM=contour` | OSC 9;4 native |
| foot | `TERM=foot*` | OSC 9;4 native |
| Rio | `TERM_PROGRAM=rio` | OSC 9;4 native |
| tmux | `TMUX` env var | DCS passthrough |
| Other TTY | Is a TTY | ASCII fallback `[====>    ] 50%` |
| Non-TTY (pipe, file) | Not a TTY | Silent (no output) |

## Environment Variables

| Variable | Effect |
| --- | --- |
| `TERMPULSE_FORCE=1` | Force OSC mode regardless of detection |
| `TERMPULSE_DISABLE=1` | Disable OSC, use ASCII fallback or silent |
| `NO_COLOR` | Avoid escape sequences, use ASCII fallback ([no-color.org](https://no-color.org/)) |

## Project Structure

```
termpulse/
├── Cargo.toml                          # Workspace root (shared deps, lints, metadata)
├── rust-toolchain.toml                 # Pins stable + rustfmt + clippy
├── .github/workflows/ci.yml           # CI: test, clippy, fmt, doc (Linux/macOS/Windows)
├── crates/
│   ├── termpulse-core/                 # no_std, zero deps
│   │   └── src/
│   │       ├── osc.rs                  # OscSequence, ProgressState, Terminator
│   │       ├── parse.rs                # find_sequences() — zero-alloc parser
│   │       ├── sanitize.rs             # sanitize_label() — injection prevention
│   │       └── strip.rs               # strip_sequences() — remove OSC from text
│   ├── termpulse/                      # Main library
│   │   └── src/
│   │       ├── controller.rs           # High-level Controller API
│   │       ├── detect.rs               # Terminal + multiplexer detection
│   │       ├── throttle.rs             # 150ms rate limiter + dedup
│   │       ├── estimate.rs             # ETA estimation (EMA algorithm)
│   │       └── backend/                # OSC, tmux, ASCII, silent backends
│   └── termpulse-cli/                  # CLI binary
│       ├── src/cmd/                    # set, start, done, fail, wrap, pipe, clear, detect
│       └── tests/cli_integration.rs    # 20 integration tests (assert_cmd)
├── CHANGELOG.md
├── CONTRIBUTING.md                     # Contribution guidelines
├── AGENTS.md                           # Developer guidelines
├── LICENSE                             # MIT
└── README.md
```

## Design Decisions

- **OSC 9;4 over custom protocols** — OSC 9;4 is the widest-supported terminal progress protocol, originated by ConEmu and adopted by Ghostty, WezTerm, iTerm2, Kitty, Windows Terminal, and others
- **stderr, not stdout** — progress output goes to stderr so it never corrupts piped data; `pipe` command passes stdin to stdout untouched
- **Best-effort writes** — write errors to the terminal are silently ignored; progress is informational, not critical
- **150ms throttle** — balances visual smoothness with terminal performance; state and label changes bypass the timer
- **Conservative multiplexer support** — tmux passthrough is enabled (well-supported since tmux 3.3+); GNU screen passthrough is disabled (too unreliable across versions)

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for release history.

## Acknowledgments

Inspired by [osc-progress](https://github.com/steipete/osc-progress).

## License

[MIT](LICENSE)
