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
# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
# Tokio runtime
tokio = { version = "1.35.1", features = ["full"] }

# CLI
clap = { version = "4.4.18", features = ["derive"] }

# Type erasure for async trait methods
async-trait = "0.1.77"

# A generic serialization/deserialization framework
serde = { version = "1.0.195", features = ["derive"] }
serde_json = "1.0.111"
serde_with = "3.4.0"

# Web framework that focuses on ergonomics and modularity
axum = "0.7.4"

# Tower middleware and utilities for HTTP clients and servers
tower-http = { version = "0.5.1", features = ["fs"] }

# Cookie manager middleware for tower.
tower-cookies = "0.10.0"

# lazy static regular expressions checked at compile time
lazy-regex = "3.1.0"

# Helpful macros for working with enums and strings
strum_macros = "0.25.3"

# A library to generate and parse UUIDs.
uuid = { version = "1.7.0", features = ["v4", "fast-rng"] }

[dev-dependencies]
# Flexible concrete Error type built on std::error::Error
anyhow = "1.0.79"

# Minimalistic HTTP Client Test Utilities
httpc-test = "0.1.8"
```

4. Add configuration variables:

```bash
pulumi config set aws:region eu-west-3
pulumi config set keypair jarvis
pulumi config set vpcNetworkCidr 192.168.110.0/24
```

> [!IMPORTANT]
> Also, add the following in `Pulumi.dev.yaml` file:

```yaml
environment:
  - aws-jarvis
```

5. Run the following command to run the program:

```bash
pulumi up
```

6. Run the following command in remote server to setup desktop gui:

```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/mohammadzainabbas/pulumi-labs/main/hack-lab-aws-python/scripts/setup_desktop.sh)"
```

> [!NOTE]
> This will take some time, so be patient.