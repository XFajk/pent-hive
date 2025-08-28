# Basic Usage Example

This example ties together a Hive and a Bee to demonstrate a simple run.

Hive: `hives/docker-local.yaml`

```yaml
name: docker-local
type: docker
docker:
  image: vulnerables/web-dvwa
  ports:
    - "8080:80"
```

Bee: `bees/simple_check.yaml`

```yaml
description: Check DVWA home
steps:
  - curl -sS http://localhost:8080/ | grep -i dvwa
```

Run:

```bash
queen run bees/simple_check.yaml --hive hives/docker-local.yaml
```
