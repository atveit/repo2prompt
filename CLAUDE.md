# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands
- Build: `cargo build`
- Run: `cargo run -- [OPTIONS]`
- Test all: `cargo test`
- Test specific: `cargo test test_name`
- Lint: `cargo clippy`
- Format: `cargo fmt`

## Code Style
- Use Rust 2021 edition
- Follow standard Rust naming conventions (snake_case for functions/variables, CamelCase for types)
- Handle errors with anyhow::Result and thiserror::Error
- Group imports by module (std, external crates, internal)
- Prefer Result<T> with context() for error handling
- Use logging (log::info, log::debug, etc.) for runtime information
- Document public functions and structs with rustdoc comments
- Keep functions small and focused on a single task

## Repository Guidelines
- **Cargo.lock**: Should be committed for binary projects (this is a binary)
- **target/** directory: Should not be committed (add to .gitignore)
- Recommended .gitignore for Rust:
  ```
  /target/
  **/*.rs.bk
  *.pdb
  .DS_Store
  ```
- Run cargo check or cargo clippy before committing changes