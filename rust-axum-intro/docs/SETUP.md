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

2. Install dependencies and dev-dependencies:

```bash

cargo add tower-http -F fs
cargo add tower-cookies lazy-regex strum_macros
cargo add uuid -F uuid/v4,fast-rng
```

> [!IMPORTANT]
> Change anything if you need to, if you want to keep the default values, just press <kbd> <br> Enter <br> </kbd> and it'll be skipped.

3. Activate the virtual environment and install new dependencies:

```bash
source venv/bin/activate
pip install pulumi-aws pulumi-awsx pulumi-random pulumi-command
```

> [!CAUTION]
> Manually, add `pulumi-aws`, `pulumi-awsx`, `pulumi-random` and `pulumi-command` (see below) in `requirements.txt` file, if you want to use `pip install -r requirements.txt` command later.

```console
pulumi>=3.0.0,<4.0.0
pulumi-aws>=6.0.0,<7.0.0
pulumi-awsx>=2.0.0,<3.0.0
pulumi-random>=3.0.0,<5.0.0
pulumi-command>=0.5.0,<1.0.0
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