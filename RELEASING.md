# Releasing termpulse

This document describes how to publish a new release.

## Pre-release checklist

1. **Ensure CI is green** on the `main` branch.

2. **Run the full local check:**

   ```bash
   just check-all
   just test-features
   just check-no-std
   ```

3. **Check semver compatibility** (requires `cargo-semver-checks`):

   ```bash
   just semver
   ```

4. **Dry-run publish** to catch packaging issues:

   ```bash
   just publish-dry-run
   ```

## Version bump

All three crates share a workspace version. Update it in a single place:

```toml
# Cargo.toml (workspace root)
[workspace.package]
version = "X.Y.Z"
```

Also update `html_root_url` in each crate's `lib.rs`:

- `crates/termpulse-core/src/lib.rs`
- `crates/termpulse/src/lib.rs`

## Update CHANGELOG

1. Move items from `[Unreleased]` into a new `[X.Y.Z] - YYYY-MM-DD` section.
2. Add a comparison link at the bottom:

   ```markdown
   [X.Y.Z]: https://github.com/justinhuangcode/termpulse/compare/vPREV...vX.Y.Z
   ```

3. Update the `[unreleased]` link to compare from the new tag.

## Tag and push

```bash
git add -A
git commit -m "release: vX.Y.Z"
git tag vX.Y.Z
git push origin main --tags
```

## What happens automatically

The `release.yml` workflow will:

1. **Build** binaries for 5 targets (Linux x86_64/aarch64, macOS x86_64/aarch64, Windows x86_64).
2. **Publish** crates to crates.io in dependency order:
   - `termpulse-core`
   - `termpulse`
   - `termpulse-cli`
3. **Create** a GitHub Release with auto-generated release notes and attached binaries.

## Post-release verification

1. Check [crates.io](https://crates.io/crates/termpulse) for the new version.
2. Check [docs.rs](https://docs.rs/termpulse) for documentation builds.
3. Verify the GitHub Release page has all 5 binary archives.
4. Test installation: `cargo install termpulse-cli`.

## Publish order (manual fallback)

If automated publishing fails, publish manually in this order:

```bash
cargo publish -p termpulse-core
cargo publish -p termpulse
cargo publish -p termpulse-cli
```

Each crate depends on the previous one, so order matters.
