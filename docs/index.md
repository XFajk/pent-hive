# Pent-Hive Documentation

Pent-Hive is a structured way to organize, automate, and reproduce pentests using YAML configurations.

## Quick Start

1. **Initialize** a new project:

   ```bash
   pent-hive init --with-schemas --with-vscode
   ```

2. **Create** your configurations:
   - **Bees** (`bees/*.bee.yaml`) - Individual tasks and commands
   - **Hives** (`hives/*.hive.yaml`) - Environments where tasks run
   - **Queens** (`queens/*.queen.yaml`) - Workflows that orchestrate everything
   - **Secrets** (`.env` file) - Sensitive data like passwords and keys

3. **Validate** your setup:

   ```bash
   pent-hive lint
   ```

4. **Test** your workflow (dry-run first):

   ```bash
   pent-hive test my-test --dry-run
   pent-hive test my-test --allow-network
   ```

## Core Concepts

- **Bees** = What to do (tasks, commands, tools)
- **Hives** = Where to do it (local, remote, containers)
- **Queens** = How to coordinate it all (workflows, dependencies)

## Documentation

- [How It Works](documentation/how_it_works.md) - Understanding the system
- [CLI Reference](documentation/cli_docs.md) - Command-line usage
- [API Specs](documentation/api/) - YAML file formats

