# Quick Start

This quick start walks through a minimal workflow: define a Hive (local docker), define a Bee (simple check), then run it with the Queen CLI.

1) Create a Hive: `hives/docker-local.yaml`

```yaml
type: docker
name: docker-local
docker:
  image: vulnerables/web-dvwa
  ports:
    - "8080:80"
```

2) Create a Bee: `bees/simple_check.yaml`

```yaml
description: Simple HTTP reachability check
steps:
  - curl -sS http://localhost:8080/ | grep -i dvwa
```

3) Run the bee with the hive:

```bash
queen run bees/simple_check.yaml --hive hives/docker-local.yaml
```

Notes:
- Use `queen validate <file>` to lint YAML files before running.
- Use `--dry-run` to show planned actions without executing.
