# Queen Specialization

Queen orchestrates Bees and Hives. A Queen file can declare workflows composed of Bees and target Hives.

Fields:

- `workflow`: ordered list of steps. Each step can be a bee reference or an inline bee.
- `concurrency` (optional): integer to run steps in parallel where allowed.
- `retry_policy` (optional): object describing retry count and backoff.

Example:

```yaml
workflow:
  - bee: bees/setup_db.yaml
  - bee: bees/run_exploit.yaml
  - bee: bees/collect_artifacts.yaml
concurrency: 1
retry_policy:
  retries: 2
  backoff: 5s
```

Notes:

- Queen files help orchestrate complex scenarios and make runs reproducible.
