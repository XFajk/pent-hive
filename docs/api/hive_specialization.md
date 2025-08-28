# Hive Specialization

Hive defines the environment where Bees run. Top-level fields:

- `name` or `id`: string — unique name.
- `type`: enum — one of `local`, `docker`, or `remote`.

Docker-specific fields (when `type: docker`):

- `image`: string — docker image name.
- `ports`: list of port mappings `host:container`.
- `env`: map of environment variables.
- `volumes` (optional): list of `host:container` mounts.

Remote-specific fields (when `type: remote`):

- `host`: string
- `user`: string
- `auth`: object — recommendations: prefer `ssh_key_path` over plaintext passwords.

Example (docker):

```yaml
name: docker-local
type: docker
docker:
  image: vulnerables/web-dvwa
  ports:
    - "8080:80"
  env:
    MYSQL_ROOT_PASSWORD: example
```

Validation & edge cases:

- Ensure port mappings are valid and do not collide.
- For `remote` hives, commands will be executed via SSH; validate connectivity during `queen validate`.
