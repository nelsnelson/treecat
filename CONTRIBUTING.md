# Contributing

This project only documents local development build steps.

## Prerequisites

- Rust toolchain >= 1.85 (Rust 2024 edition)
- Cargo (comes with Rust)

Optional but recommended:
- rustfmt (Rust formatter)
- clippy (Rust linter)

## Build

From the repo root:

```sh
cargo build
````

Release build:

```sh
cargo build --release
```

## Run

Run against the current directory:

```sh
cargo run
```

Run against specific paths:

```sh
cargo run -- path1 path2
```

Tip: pass `--` before paths to avoid Cargo interpreting them as its own flags.

## Format

```sh
cargo fmt
```

## Lint

```sh
cargo clippy -- -D warnings
```

## Test

```sh
cargo test
```
