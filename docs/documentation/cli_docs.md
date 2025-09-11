# CLI Reference

The Pent-Hive CLI has four main commands: `init`, `lint`, `build`, and `test`.

## Commands

### init

Create a new Pent-Hive project:

```bash
pent-hive init [--with-schemas] [--with-vscode]
```

This creates the basic folder structure:

- `bees/` - Task definitions
- `hives/` - Environment definitions  
- `queens/` - Workflow definitions

Options:

- `--with-schemas` - Install JSON schema files locally
- `--with-vscode` - Configure VS Code for YAML validation

### lint

Validate all your YAML files:

```bash
pent-hive lint           # Check everything
pent-hive lint file.yaml # Check one file
```

Catches:

- Invalid YAML syntax
- Schema violations
- Missing references (Queen → Bee, Queen → Hive)
- Circular dependencies
- Hardcoded secrets (use `@secret(key)` instead)

### build

Build tools and dependencies defined in Bees:

```bash
pent-hive build [--parallel] [--bees pattern]
```

Options:

- `--parallel` - Build multiple Bees concurrently
- `--bees pattern` - Only build matching Bees

### test

Run workflows defined by Queens:

```bash
pent-hive test workflow-name [options...]
```

> [!WARNING]
> Always run with `--dry-run` first to see what will happen.

Important options:

- `--dry-run` - Show the plan without executing
- `--allow-network` - Enable network operations
- `--consent id` - Required for remote targets

Example workflow:

```bash
# 1. Set up your secrets
echo "TARGET_IP=192.168.1.100" > .env
echo "SSH_USER=pentester" >> .env
echo "API_TOKEN=sk-1234567890" >> .env

# 2. Preview what will happen (secrets will show as [MASKED])
pent-hive test my-workflow --dry-run

# 3. Run for real (if everything looks good)
pent-hive test my-workflow --allow-network --consent consent-2025-xyz
```

## Global Options

These work with any command:

- `--project-dir path` - Use a different project folder
- `--quiet` - Less output
- `--verbose` - More debug info

## Safety Features

- Network operations are blocked by default (use `--allow-network`)
- Remote targets require explicit consent IDs
- Secrets are masked in dry-run output
- Everything is validated before execution

