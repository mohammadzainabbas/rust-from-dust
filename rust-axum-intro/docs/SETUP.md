# How this project was setup ?

> [!NOTE]
> Follow the steps below to setup this project from scratch.

## Key Concepts

- [x] Add dependencies and dev-dependencies in `Cargo.toml` file


### Setup

1. Create an empty Rust project:

```bash
cargo new rust-axum-intro
cd rust-axum-intro
```

2. Install dependencies:

```bash
cargo add tokio -F full
cargo add clap -F derive
cargo add serde serde_json serde_with -F serde/derive
cargo add tower-http -F fs
cargo add async-trait axum tower-cookies lazy-regex strum_macros
cargo add uuid -F v4,fast-rng # or cargo add uuid -F uuid/v4,fast-rng
```

> [!IMPORTANT]
> Use `-F` or `--features` to add additional features. For example, `cargo add uuid -F v4,fast-rng` will add `uuid` crate with `v4` and `fast-rng` features.

3. Install dev-dependencies:

```bash
cargo add anyhow httpc-test --dev
```

> [!IMPORTANT]
> Use `--dev` to add dev-dependencies.

4. Add additional properties in `Cargo.toml` file, see below the entire `Cargo.toml` file:

```toml
[package]
name = "rust-axum-intro"
version = "0.1.0"
edition = "2021"
description = "A Rust backend project based on Axum web framework."
authors = ["Mohammad Zain Abbas <mohammadzainabbas@gmail.com>"]
homepage = "https://github.com/mohammadzainabbas/rust-from-dust/tree/main/rust-axum-intro"
repository = "https://github.com/mohammadzainabbas/rust-from-dust/tree/main/rust-axum-intro"
keywords = ["rust", "rust-axum", "backend", "axum", "web-application"]
categories = ["command-line-utilities"]
```
> [!TIP]
> See more [_keys_ and their _definitions_](https://doc.rust-lang.org/cargo/reference/manifest.html).

<!-- 5.  -->