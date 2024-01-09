# Hello World (with Cargo)

[![Rust Badge](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)](https://play.rust-lang.org/) [![Rust Report Card](https://rust-reportcard.xuri.me/badge/github.com/mohammadzainabbas/rust-from-dust)](https://github.com/mohammadzainabbas/rust-from-dust/tree/main/hello-world) [![MIT License](https://badgen.net/github/license/mohammadzainabbas/rust-from-dust?icon=github)](https://github.com/mohammadzainabbas/rust-from-dust?tab=MIT-1-ov-file)

## Overview

A tutorial for <kbd> <br> Hello World! <br> </kbd> based program built via `Cargo`: _a Rust’s build system and package manager_.

> [!TIP]
> This tutorial is based on [Rust Book](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html) and [Rust By Example](https://doc.rust-lang.org/rust-by-example/hello.html).

## Key Concepts

- [x] Create new binary project using `cargo new hello-world`
- [x] Build & run the project using `cargo run`

## Prerequisites

* `Rust` _(stable, with cargo installed)_

## Quick Start

### Setup

1. Create a new binary project using `cargo new hello-world`:

```bash
cargo new hello-world
```

or 

```bash
cargo new hello-world --bin
```

> [!NOTE]
> By default, `cargo new` creates a binary (application) template. You can use `--bin` flag to explicitly specify it.

2. Build & run the project using `cargo run`:

```bash
cd hello-world
cargo run
```

> [!TIP]
> Run `cargo run --help` or `cargo help run` to see detailed options of `cargo run` command.

You should see the following output:

```console
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/hello-world`
Hello, world!
```

> [!IMPORTANT] 
> `cargo run` command builds the project if it's not already built.
