# Hello World (with Cargo)

[![Rust Badge](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)](https://github.com/mohammadzainabbas/rust-from-dust/tree/main/hello-world) [![Rust Report Card](https://rust-reportcard.xuri.me/badge/github.com/mohammadzainabbas/rust-from-dust)](https://github.com/mohammadzainabbas/rust-from-dust/tree/main/hello-world) 

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

1. Configuring OpenID Connect for AWS:

Follow the guideline [here](https://www.pulumi.com/docs/pulumi-cloud/oidc/aws/) to configure `Pulumi` to use OpenID Connect to authenticate with AWS.

2. Clone the repo:

```bash
git clone https://github.com/mohammadzainabbas/pulumi-labs.git
```

or if GitHub CLI is installed:

```bash
gh repo clone mohammadzainabbas/pulumi-labs
```

3. Change directory:

```bash
cd aws-fleet-python
```

4. Create a new Python virtualenv, activate it, and install dependencies:

```bash
python3 -m venv venv
source venv/bin/activate
pip3 install -r requirements.txt
```

5. Create a new Pulumi stack, which is an isolated deployment target for this example:

```bash
pulumi stack init
```

6. Update your environment:

Now, update your environment (that you'd already setup in step 1) in `Pulumi.dev.yaml` like the following:

```yaml
environment:
  - aws-jarvis
```

> [!NOTE]
> Here, `aws-jarvis` is the name of the environment that I've created in step 1.

7. Set the AWS region (optional):

To deploy to a region other than the default one configured for your AWS CLI profile, run `pulumi config set aws:region <region>`

```bash
pulumi config set aws:region us-east-1
```

> [!IMPORTANT] 
> If you don't specify anything, everything will be deployed in `eu-west-3` region.

8. Run `pulumi up` to preview and deploy changes:

```bash
pulumi up
```

> [!NOTE] 
> You can use `--yes` flag to skip the confirmation prompt.

and voila! You've deployed Auto scaling group using spot fleet along with your custom launch config to AWS.

### Cleanup

To destroy the Pulumi stack and all of its resources:

```bash
pulumi destroy
```

> [!NOTE] 
> You can use `--yes` flag to skip the confirmation prompt.