# termpulse-cli

[![Crates.io](https://img.shields.io/crates/v/termpulse-cli?style=flat-square&logo=rust&logoColor=white)](https://crates.io/crates/termpulse-cli)
[![docs.rs](https://img.shields.io/docsrs/termpulse-cli?style=flat-square&logo=docs.rs&logoColor=white)](https://docs.rs/termpulse-cli)
[![License](https://img.shields.io/crates/l/termpulse-cli?style=flat-square)](../../LICENSE)

CLI for native terminal progress indicators via OSC 9;4.

## Installation

```bash
cargo install termpulse-cli
```

## Quick Start

```bash
# Set progress to 50%
termpulse set 50 -l "Building"

# Indeterminate spinner
termpulse start -l "Compiling"

# Wrap a command with automatic progress
termpulse wrap -- cargo build --release

# Pipe with progress tracking
curl -sL https://example.com/file.tar.gz \
  | termpulse pipe --total 104857600 -l "Downloading" \
  > file.tar.gz

# Signal completion
termpulse done -l "Build complete"

# Detect terminal capabilities
termpulse detect --json
```

## Commands

| Command | Description |
| --- | --- |
| `set <percent> [-l label]` | Set progress percentage (0-100) |
| `start [-l label]` | Start indeterminate progress |
| `done [-l label]` | Signal success |
| `fail [-l label]` | Signal failure |
| `wrap -- <command...>` | Wrap a shell command with progress |
| `pipe [--total N] [--lines]` | Pipe stdin to stdout with progress |
| `clear` | Clear the progress indicator |
| `detect` | Show terminal capabilities |
| `completions <shell>` | Generate shell completions |

## Shell Completions

```bash
# Bash
termpulse completions bash > ~/.local/share/bash-completion/completions/termpulse

# Zsh
termpulse completions zsh > ~/.zfunc/_termpulse

# Fish
termpulse completions fish > ~/.config/fish/completions/termpulse.fish
```

## Part of termpulse

This is the CLI binary. For the Rust library, see [termpulse](https://crates.io/crates/termpulse). For the low-level `no_std` core, see [termpulse-core](https://crates.io/crates/termpulse-core).

## License

[MIT](../../LICENSE)
