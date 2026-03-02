# Contributing to termpulse

Thank you for your interest in contributing! This document covers everything
you need to get started.

## Prerequisites

- Rust 1.85+ (MSRV)
- Git
- [just](https://github.com/casey/just) (optional, recommended)

## Getting Started

```bash
git clone https://github.com/justinhuangcode/termpulse.git
cd termpulse
just test                # or: cargo test --workspace
```

## Development Workflow

A [justfile](justfile) is provided for common tasks. Install `just` with
`cargo install just`, then use the recipes below. Equivalent `cargo` commands
are shown for contributors who prefer not to install `just`.

### Running Tests

```bash
just test                # or: cargo test --workspace
just test-features       # or: cargo test -p termpulse-core --no-default-features
                         #     cargo test -p termpulse-core --all-features
just check-no-std        # or: cargo check -p termpulse-core --target thumbv7m-none-eabi
```

### Linting

```bash
just clippy              # or: cargo clippy --workspace --all-targets -- -D warnings
just fmt                 # or: cargo fmt --all -- --check
```

### Benchmarks

```bash
just bench               # or: cargo bench -p termpulse-core
```

### Documentation

```bash
just doc                 # or: RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
```

### Run All Checks

```bash
just check-all           # runs test, clippy, fmt, doc in sequence
```

## Project Structure

```
crates/
  termpulse-core/   # no_std, zero deps — protocol only
  termpulse/        # std library — detection, backends, controller
  termpulse-cli/    # binary — user-facing CLI
```

Changes should target the **narrowest crate** possible:

- Protocol parsing/encoding → `termpulse-core`
- Terminal detection, backends → `termpulse`
- CLI commands, flags → `termpulse-cli`

## Code Style

- Follow existing patterns — the workspace enforces `clippy::all` and
  `rust_2018_idioms`
- `unsafe` code is denied at the workspace level
- All public items must have doc comments (`#![warn(missing_docs)]`)
- Use `cargo fmt` before committing

## Commit Messages

Use clear, imperative-mood messages:

```
fix: sanitize_label not idempotent when truncating before whitespace
feat: add tmux DCS passthrough backend
test: add proptest for write/parse round-trip
```

## Pull Request Process

1. Fork the repository and create a feature branch
2. Make your changes with tests
3. Ensure all checks pass: `just check-all` (or run test, clippy, fmt, doc manually)
4. Open a PR against `main`
5. Fill out the PR template

## Reporting Bugs

Use the [bug report template](https://github.com/justinhuangcode/termpulse/issues/new?template=bug_report.yml).

## Security Vulnerabilities

See [SECURITY.md](SECURITY.md) for responsible disclosure procedures.
**Do not open public issues for security vulnerabilities.**

## License

By contributing, you agree that your contributions will be licensed under the
MIT License.
