# CLI Overview

The `queen` CLI is the primary interface for running Bees against Hives and orchestrating workflows.

Key commands:

- `queen run <bee> --hive <hive>` — execute a Bee with a Hive
- `queen init` — scaffold a new Bee or Hive
- `queen validate <file>` — validate YAML against the API contract
- `queen list` — list known bees and hives

Common flags: `--dry-run`, `--debug`, `--output`, `--concurrency`.
