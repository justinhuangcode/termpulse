# AGENTS.md — Developer Guidelines for termpulse

## Project Structure

```
termpulse/
├── Cargo.toml                 # Workspace root (shared deps, lints, metadata)
├── crates/
│   ├── termpulse-core/        # no_std, zero deps — OSC 9;4 primitives
│   │   └── src/
│   │       ├── lib.rs         # Public API exports
│   │       ├── osc.rs         # Sequence construction (OscSequence, ProgressState)
│   │       ├── parse.rs       # Sequence parser (find_sequences)
│   │       ├── sanitize.rs    # Label sanitization (injection prevention)
│   │       └── strip.rs       # Strip sequences from text
│   ├── termpulse/             # Main library
│   │   └── src/
│   │       ├── lib.rs         # Public API, re-exports core types
│   │       ├── controller.rs  # High-level Controller API
│   │       ├── detect.rs      # Terminal capability + multiplexer detection
│   │       ├── throttle.rs    # Rate limiting & deduplication
│   │       ├── estimate.rs    # ETA estimation (EMA algorithm)
│   │       └── backend/       # Output backends
│   │           ├── mod.rs     # Backend trait
│   │           ├── osc.rs     # Native OSC 9;4 output
│   │           ├── tmux.rs    # DCS passthrough for tmux
│   │           ├── ascii.rs   # ASCII progress bar fallback
│   │           └── silent.rs  # No-op backend
│   └── termpulse-cli/         # CLI binary
│       └── src/
│           ├── main.rs        # Entry point, command dispatch
│           ├── cli.rs         # clap argument definitions
│           ├── output.rs      # Output formatting utilities
│           └── cmd/           # Command handlers
│               ├── set.rs, start.rs, done.rs, fail.rs
│               ├── wrap.rs    # Wrap child process with progress
│               ├── pipe.rs    # Pipe stdin→stdout with progress
│               ├── clear.rs, detect.rs
└── .github/workflows/ci.yml  # CI: test, clippy, fmt, doc
```

## Design Principles

1. **Core narrow, outer wide** — `termpulse-core` is `#![no_std]` with zero deps; outer crates add functionality
2. **Dependency injection** — All I/O goes through traits (`Backend`, `EnvLookup`, `Write`) for testability
3. **No panic** — All public APIs are infallible from the caller's perspective
4. **`forbid(unsafe_code)`** — Enforced across all crates
5. **Three-tier fallback** — OSC → ASCII → Silent; never crash on unsupported terminals

## Coding Conventions

- **Edition**: Rust 2024, MSRV 1.85
- **Lints**: `clippy::all` (warn), `clippy::correctness` (deny), `unsafe_code` (deny)
- **CLI I/O**: `#![allow(clippy::print_stdout, clippy::print_stderr)]` only in `termpulse-cli`
- **Tests**: Every module has `#[cfg(test)] mod tests` at the bottom
- **Doc-tests**: All public functions have `/// # Example` doc-tests
- **Labels**: Always sanitize via `sanitize_label()` before embedding in OSC sequences
- **Percentages**: Always clamp to 0-100 via `.min(100)`

## Testing

```bash
# Run all tests
cargo test --workspace

# Check lints
cargo clippy --workspace --all-targets -- -D warnings

# Check formatting
cargo fmt --all -- --check

# Build docs
cargo doc --workspace --no-deps
```

## Adding a New Terminal

1. Add detection heuristic in `crates/termpulse/src/detect.rs` → `detect_with_env()`
2. Add a test with `MockEnv` in the same file
3. Update the table in `README.md`
4. Update the doc comment on `pub fn detect()`

## Adding a New Backend

1. Create `crates/termpulse/src/backend/<name>.rs`
2. Implement the `Backend` trait
3. Add `pub mod <name>;` to `backend/mod.rs`
4. Wire it into `Controller::with_options()` in `controller.rs`

## Adding a New CLI Command

1. Add variant to `Command` enum in `cli.rs`
2. Add opts struct (e.g., `FooOpts`) in `cli.rs`
3. Create `cmd/foo.rs` with `pub fn run(opts, json) -> Result<()>`
4. Add `pub mod foo;` to `cmd/mod.rs`
5. Add match arm in `main.rs`

## Key Invariants

- `termpulse-core` must never gain dependencies (check `Cargo.toml`)
- `termpulse-core` must remain `#![no_std]` compatible
- No `vec!` or heap allocation in `termpulse-core` tests (use fixed-size arrays)
- Multi-byte UTF-8 labels must be handled safely (use `char_indices()`, not byte indices)
- The `\u{009c}` (C1 ST) character is 2 bytes in UTF-8 — tests use `\u{009c}` not `\x9c`
- `NO_COLOR` env var must be respected (falls back to ASCII, not OSC)
- `TERMPULSE_FORCE` takes priority over `NO_COLOR`
- Examples need `#![allow(clippy::print_stdout, clippy::print_stderr)]`
- Integration tests live in `crates/termpulse-cli/tests/`
- Examples live in `crates/termpulse/examples/`
