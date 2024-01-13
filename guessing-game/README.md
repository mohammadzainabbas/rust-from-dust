# Guessing Game

[![Rust Badge](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)](https://play.rust-lang.org/) [![Rust Report Card](https://rust-reportcard.xuri.me/badge/github.com/mohammadzainabbas/rust-from-dust)](https://github.com/mohammadzainabbas/rust-from-dust/tree/main/guessing-game) [![MIT License](https://badgen.net/github/license/mohammadzainabbas/rust-from-dust?icon=github)](https://github.com/mohammadzainabbas/rust-from-dust?tab=MIT-1-ov-file)

## Overview

A **Rust** program to play two _guessing_ games: <kbd> <br> Guess the number <br> </kbd> and <kbd> <br> Guess the word <br> </kbd> - built via `Cargo`: _a Rust’s build system and package manager_.

> [!NOTE]
> Below mentioned video on _YouTube_ was used as a starting point for this project. 
> [![Programming a Guessing Game in Rust!](http://img.youtube.com/vi/H0xBSbnQYds/0.jpg)](http://www.youtube.com/watch?v=H0xBSbnQYds)

## Key Concepts

- [x] Create new binary project using `cargo new guessing-game`
- [x] Use of external libraries `rand`, `colored` and `dialoguer`
- [x] Check program without running `cargo check`
- [x] Fix issues with program without running `cargo fix`
- [x] Build & run the project using `cargo run`

## Prerequisites

> [!IMPORTANT]
> Make sure you have `Rust` _(stable, with cargo)_ installed on your system. If not, follow the instructions [here](https://www.rust-lang.org/tools/install).

### Quick Start

> [!NOTE]
> Follow below mentioned steps to run the project on your local machine.

1. Clone the repository and navigate to `guessing-game` directory:

```bash
git clone https://github.com/mohammadzainabbas/rust-from-dust.git
cd rust-from-dust/guessing-game
```

or

```bash
gh repo clone mohammadzainabbas/rust-from-dust
cd rust-from-dust/guessing-game
```

2. Create new files `src/main.rs` and `src/lib.rs`:

```bash
cd guessing-game
touch src/main.rs src/lib.rs
```

```bash
cd guessing-game
cargo run
```

> [!TIP]
> Run `cargo run --help` or `cargo help run` to see detailed options of `cargo run` command.

You should see the following output:

```console
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/guessing-game`
Hello, world!
```

> [!IMPORTANT] 
> `cargo run` compiles/builds the code and then run the resultant executable all in one command.
