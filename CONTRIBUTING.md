# Contributing to termpulse

Thank you for your interest in contributing! This document covers everything
you need to get started.

## Prerequisites

- Rust 1.85+ (MSRV)
- Git

## Getting Started

```bash
git clone https://github.com/justinhuangcode/termpulse.git
cd termpulse
cargo test --workspace
```

## Development Workflow

### Running Tests

```bash
# All tests
cargo test --workspace

# Specific crate
cargo test -p termpulse-core

# Property-based tests (slow, more iterations)
cargo test -p termpulse-core --test proptest_core
```

### Linting

```bash
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all -- --check
```

### Benchmarks

```bash
cargo bench -p termpulse-core
```

### Documentation

```bash
cargo doc --workspace --no-deps --open
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
3. Ensure all checks pass: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --all -- --check`
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
