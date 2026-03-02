# termpulse

[![Crates.io](https://img.shields.io/crates/v/termpulse?style=flat-square&logo=rust&logoColor=white)](https://crates.io/crates/termpulse)
[![docs.rs](https://img.shields.io/docsrs/termpulse?style=flat-square&logo=docs.rs&logoColor=white)](https://docs.rs/termpulse)
[![License](https://img.shields.io/crates/l/termpulse?style=flat-square)](../../LICENSE)

Native terminal progress indicators via OSC 9;4 -- with smart terminal detection, graceful fallback, throttling, and ETA estimation.

## Usage

```rust
use termpulse::Controller;

let mut ctrl = Controller::auto();
ctrl.set(25, "Downloading");
ctrl.set(50, "Downloading");
ctrl.set(75, "Downloading");
ctrl.done("Complete");
```

## Features

- **Zero-config detection** -- auto-detects 10+ terminals (Ghostty, WezTerm, iTerm2, Kitty, Windows Terminal, VS Code, and more)
- **Three-tier fallback** -- OSC 9;4 native > ASCII progress bar > silent mode
- **tmux DCS passthrough** -- automatic multiplexer detection and wrapping
- **Throttle + dedup** -- 150ms rate limiting with deduplication
- **ETA estimation** -- exponential moving average algorithm
- **Label sanitization** -- prevents escape sequence injection
- **`NO_COLOR` support** -- respects [no-color.org](https://no-color.org/) convention
- **Dependency injection** -- all I/O is injectable via traits for testing

## Part of termpulse

This is the main library crate. For the low-level `no_std` core, see [termpulse-core](https://crates.io/crates/termpulse-core). For the CLI, see [termpulse-cli](https://crates.io/crates/termpulse-cli).

## License

[MIT](../../LICENSE)
