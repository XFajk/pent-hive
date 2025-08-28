# Commands Reference

`queen run`

Description: Run a Bee with a specified Hive.

Examples:

```bash
queen run bees/example.yaml --hive hives/docker-local.yaml
queen run bees/example.yaml --hive hives/docker-local.yaml --dry-run --debug
```

`queen validate`

Description: Validate Bee/Hive/Queen YAML files against the documented schema; useful to keep API contracts stable.

`queen init`

Description: Scaffold new templates for Bees and Hives in the current repo.
