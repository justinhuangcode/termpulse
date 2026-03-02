# termpulse development tasks
# Install just: cargo install just

# Run all checks (test, clippy, fmt, doc)
check-all: test clippy fmt doc
    @echo "All checks passed."

# Run all tests
test:
    cargo test --workspace

# Run clippy with deny warnings
clippy:
    cargo clippy --workspace --all-targets -- -D warnings

# Check formatting
fmt:
    cargo fmt --all -- --check

# Build docs with warnings as errors
doc:
    RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps

# Run tests with all feature combinations for termpulse-core
test-features:
    cargo test -p termpulse-core --no-default-features
    cargo test -p termpulse-core --all-features

# Verify no_std compatibility
check-no-std:
    rustup target add thumbv7m-none-eabi 2>/dev/null || true
    cargo check -p termpulse-core --target thumbv7m-none-eabi

# Run benchmarks
bench:
    cargo bench -p termpulse-core

# Dry-run publish for all crates
publish-dry-run:
    cargo publish --dry-run --allow-dirty -p termpulse-core
    cargo publish --dry-run --allow-dirty -p termpulse
    cargo publish --dry-run --allow-dirty -p termpulse-cli

# Check semver compatibility
semver:
    cargo semver-checks check-release -p termpulse-core
    cargo semver-checks check-release -p termpulse

# Format all code
fmt-fix:
    cargo fmt --all
