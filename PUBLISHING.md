# Publishing Guide for repo2prompt

This document explains how to build, test, and publish the repo2prompt crate to crates.io.

## Prerequisites

1. Install Rust and Cargo: https://www.rust-lang.org/tools/install
2. Create a crates.io account: https://crates.io/

## Testing the Package

```bash
# Build the package
cargo build

# Run tests
cargo test

# Run with example arguments
cargo run -- --format json --include "*.rs" --pretty
```

## Preparing for Publication

1. Update version number in `Cargo.toml` if needed
2. Ensure GitHub repository URL is correct in `Cargo.toml`
3. Check package metadata with:
   ```bash
   cargo package --list
   ```

## Publishing to crates.io

1. Login to crates.io:
   ```bash
   cargo login
   ```
   (You'll be prompted for your API token from https://crates.io/me)

2. Publish the crate:
   ```bash
   cargo publish
   ```

## Post-Publication

1. Create a git tag for the version:
   ```bash
   git tag -a v0.1.0 -m "Initial release"
   git push origin v0.1.0
   ```

2. Update the README with installation instructions pointing to crates.io

## Updating the Package

1. Make changes to the code
2. Update version in `Cargo.toml` (follow semantic versioning)
3. Run tests: `cargo test`
4. Publish: `cargo publish`
5. Create and push a new tag for the version