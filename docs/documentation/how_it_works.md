# How Pent-Hive Works

Pent-Hive organizes pentests into three simple components that work together.

## The Three Components

### Bees - Your Tasks

A Bee is a YAML file that describes a single task:

```yaml
pent_hive_spec: "1.0"
metadata:
  name: port_scan
execution:
  steps:
    - run: "nmap -sS @target"
```

Each Bee can:
- Run commands (`execution.steps`)
- Build tools first if needed (`build.steps`)
- Accept parameters (`@target`, `@1`, `@name`)
- Collect output files (`artifacts`)

### Hives - Your Environments

A Hive is a YAML file that describes where tasks run:

```yaml
pent_hive_spec: "1.0"
metadata:
  name: attacker
type: CONTAINER
container:
  image: kali:latest
```

Three types available:
- `HOST` - Run on your local machine
- `REMOTE` - Run on a remote machine via SSH
- `CONTAINER` - Run inside a Docker container

### Queens - Your Workflows

A Queen is a YAML file that orchestrates Bees on Hives:

```yaml
pent_hive_spec: "1.0"
metadata:
  name: basic_scan
steps:
  - id: scan
    hive: attacker
    run: ["port_scan 192.168.1.100"]
```

Queens define:
- Which Bees to run
- On which Hives
- In what order (dependencies)
- How to handle failures

## How They Connect

1. **You write** Bees (tasks), Hives (environments), and Queens (workflows)
2. **Pent-Hive validates** everything with `pent-hive lint`
3. **You preview** with `pent-hive test workflow --dry-run`
4. **You execute** with `pent-hive test workflow --allow-network`

## Parameter Flow

Parameters flow from Queens to Bees:

```yaml
# Queen calls Bee with parameter
run: ["port_scan 192.168.1.100"]

# Bee receives it as @1
run: "nmap -sS @1"
```

## Working with Secrets

Sensitive data like passwords, API keys, and SSH keys should never be hardcoded in YAML files. Use `@secret(KEY)` tokens instead.

### Setting up Secrets

Create a `.env` file in your project root:

```bash
# .env
TARGET_HOST=192.168.1.100
SSH_USERNAME=pentester
SSH_PRIVATE_KEY=/home/user/.ssh/client_key
API_TOKEN=sk-1234567890abcdef
WORDLIST_PATH=/usr/share/wordlists/rockyou.txt
DB_PASSWORD=super_secret_password
```

### Using Secrets in YAML

```yaml
# In Hives
connection:
  host: "@secret(TARGET_HOST)"
  user: "@secret(SSH_USERNAME)"
  ssh_key_ref: "@secret(SSH_PRIVATE_KEY)"

# In Bees
steps:
  - run: "curl -H 'Authorization: Bearer @secret(API_TOKEN)' @target"

# In environment variables
env:
  DB_PASS: "@secret(DB_PASSWORD)"
```

### Security Features

- Secrets are **masked** in `--dry-run` output (shows `[MASKED]`)
- Secrets are loaded at runtime, never stored in YAML
- Environment variables take precedence over `.env` file
- Secrets are never logged in plain text

> [!TIP]
> Use `@secret(key)` for sensitive data - it gets masked in dry-runs and loaded securely at runtime.

## Typical Workflow

1. **Start** with `pent-hive init`
2. **Write** your Bees, Hives, and Queens
3. **Validate** with `pent-hive lint`
4. **Preview** with `--dry-run`
5. **Execute** for real

That's it. The system handles scheduling, dependencies, and execution across different environments.
