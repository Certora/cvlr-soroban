# CVLR for Soroban

Soroban-specific components of [CVLR](https://github.com/Certora/cvlr) (Certora Verification Language for Rust), used for writing and verifying Soroban smart contracts with the Certora's Sunbeam verifier.

## Overview

This workspace provides Soroban-related utilities for writing specs for Certora's. Sunbeam verifier. It includes:

- nondeterministic constructors for common Soroban values
- logging adapters for Soroban SDK types
- verification helpers for authorization-related reasoning
- proc-macros for rules, mock clients, and Soroban compatibility shims
- helper macros for swapping native implementations with summarized verification behavior

## Workspace Crates

This repository contains three crates:

### `cvlr-soroban`

Core Soroban support for verification-oriented builds, including:

- nondeterministic `Address`, `Map`, `String`, `Vec`, `Symbol`, `Bytes`, and `BytesN<32>` constructors
- logging wrappers for `Address`, `Symbol`, `Bytes`, and `BytesN<32>` through `cvlr-log`
- helper APIs such as `is_auth` for reasoning about authorization in specs

### `cvlr-soroban-derive`

Proc-macros for Soroban verification workflows, including:

- `#[rule]` and `declare_rule!` for rule registration
- `#[cvlr_mock_client(...)]` for generating mock client implementations from traits
- `#[contractevent]` and `#[topic]` compatibility shims for CVLR builds

### `cvlr-soroban-macros`

Macro helpers used to redirect code to summarized verification behavior, including:

- `apply_summary!` for replacing function or method bodies under verification-specific configurations

## Getting Started

Add the crates you need to your `Cargo.toml`:

```toml
[dependencies]
cvlr-soroban = "0.4.0"
cvlr-soroban-derive = "0.4.0"
cvlr-soroban-macros = "0.4.0"
soroban-sdk = "25.1.1"
```

Omit any crates that your project does not use.

## Using Unreleased Versions

If you want to consume the latest unreleased version of a crate from this workspace, you can depend on it directly from GitHub instead of crates.io:

```toml
[dependencies]
cvlr-soroban = { git = "https://github.com/Certora/cvlr-soroban", branch = "main" }
cvlr-soroban-derive = { git = "https://github.com/Certora/cvlr-soroban", branch = "main" }
cvlr-soroban-macros = { git = "https://github.com/Certora/cvlr-soroban", branch = "main" }
```

## Building and Testing

Build the workspace:

```bash
cargo build --release
```

Run all tests:

```bash
cargo test
```

The proc-macro expansion tests in `cvlr-soroban-derive` use `macrotest` and `trybuild`. They require `cargo-expand`:

```bash
cargo install cargo-expand
```

Run the proc-macro test harness:

```bash
cargo test -p cvlr-soroban-derive --test test_contractevent
```

If you intentionally change macro expansion and want to refresh the snapshot files:

```bash
MACROTEST=overwrite cargo test -p cvlr-soroban-derive --test test_contractevent test_contractevent_macro_expansion
```

## Documentation and Related Repositories

- [CVLR](https://github.com/Certora/cvlr)
- [Soroban verification documentation](https://docs.certora.com/en/latest/docs/sunbeam/index.html)
- [Sunbeam tutorials](https://github.com/Certora/sunbeam-tutorials)

## License

This project is licensed under the MIT License.

## Release

Current release: `0.4.0`
