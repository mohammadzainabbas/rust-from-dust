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
cargo add tracing tracing-appender
cargo add tracing-subscriber -F env-filter,json
cargo add validator -F derive
```

> [!IMPORTANT]
> Use `-F` or `--features` to add additional features. For example, `cargo add uuid -F v4,fast-rng` will add `uuid` crate with `v4` and `fast-rng` features.

3. Install dev-dependencies:

```bash
cargo add anyhow httpc-test --dev
cargo add tower -F util --dev
cargo add http-body-util --dev
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

5. Install [`cargo watch`](https://crates.io/crates/cargo-watch), if not already installed:

```bash
cargo install cargo-watch
```

> [!TIP]
> Use [`cargo watch`](https://crates.io/crates/cargo-watch) to automatically run the project on file changes.

6. Watch _server_ and _client_ changes via `cargo watch`:

Open two terminals and run the following commands:

> #1: to watch (re-build and run) server-side changes
```bash
cargo watch -q -c -w src/ -x run
```

> #2: to watch client-side behavior via tests (re-run [`quick_dev.rs`](https://github.com/mohammadzainabbas/rust-from-dust/blob/main/rust-axum-intro/tests/quick_dev.rs) test on changes in `src/` and `tests/` directories)
```bash
cargo watch -q -c -w src/ -w tests/ -x "test -q quick_dev -- --nocapture"
```
or
```bash
cargo watch -q -c -w src/ -w tests/ -x "test -- --nocapture"
```

> [!IMPORTANT]
> The `-q` flag is used to suppress the output of `cargo watch` command. The `-c` flag is used to clear the terminal before running the command. The `-w` flag is used to watch the changes in the specified directory. The `-x` flag is used to run the command.