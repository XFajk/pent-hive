# Hive Specification

A Hive defines where Bees run. It describes an execution environment with connection details, constraints, and security settings.

## File Structure

Hive files must:

- End with `.hive.yaml`
- Start with `pent_hive_spec: "1.0"`
- Have a unique `metadata.name`

## Required Fields

```yaml
pent_hive_spec: "1.0"
metadata:
  name: my_environment
type: HOST|REMOTE|CONTAINER
```

## Hive Types

### HOST - Local Machine

Runs Bees on your local machine:

```yaml
pent_hive_spec: "1.0"
metadata:
  name: local
type: HOST
```

### REMOTE - SSH Connection

Runs Bees on a remote machine via SSH:

```yaml
pent_hive_spec: "1.0"
metadata:
  name: target_server
type: REMOTE
connection:
  host: "@secret(TARGET_HOST)"
  user: "@secret(SSH_USER)"
  ssh_key_ref: "@secret(SSH_PRIVATE_KEY)"
consent_required: true
consent_id: "pentest-2025-target-approval"
```

Required fields for REMOTE:

- `connection.host` - IP address or hostname
- `connection.user` - SSH username

Optional fields:

- `connection.port` - SSH port (default: 22)
- `connection.ssh_key_ref` - Reference to SSH key in secret store
- `connection.flags` - Additional SSH flags

### CONTAINER - Docker Container

Runs Bees inside a Docker container:

```yaml
pent_hive_spec: "1.0"
metadata:
  name: kali_attacker
type: CONTAINER
container:
  image: kali:latest
  network_mode: "host"
constraints:
  allow_privileged: true
```

Required fields for CONTAINER:

- `container.image` - Docker image name

Optional fields:

- `container.network_mode` - Docker network mode
- `container.mounts` - Volume mounts
- `container.flags` - Additional docker flags

## Metadata Section

### Required

- `name` - Unique identifier for this Hive

### Optional

- `description` - Human-readable description
- `owner` - Who owns/manages this environment
- `tags` - Array of labels (e.g., `["os:linux", "role:attacker"]`)

## Security & Constraints

Control what Bees can do on this Hive:

```yaml
constraints:
  allow_privileged: false
  allowed_networks:
    - "10.0.0.0/8"
    - "192.168.1.0/24"
  max_cpu: "500m"
  max_memory: "1Gi"
```

- `allow_privileged` - Allow privileged/root execution
- `allowed_networks` - CIDR ranges this Hive can access
- `max_cpu` - CPU limit hint
- `max_memory` - Memory limit hint

## Consent & Safety

For REMOTE Hives targeting external systems:

```yaml
consent_required: true
consent_id: "pentest-approval-2025-client-xyz"
```

- `consent_required` - Require explicit consent before execution
- `consent_id` - Reference to documented approval

> [!WARNING]
> Always require consent for remote targets you don't own.

## Secret References

Never put credentials directly in Hive files. Use secret references:

```yaml
connection:
  ssh_key_ref: "@secret(SSH_PRIVATE_KEY)"
  # NOT: ssh_key: "-----BEGIN PRIVATE KEY-----..."
```

> [!TIP]
> Secrets are loaded from environment variables or `.env` files at runtime and masked during dry-runs.

## Examples

### Local Development

```yaml
pent_hive_spec: "1.0"
metadata:
  name: localhost
  description: "Local development machine"
type: HOST
```

### Kali Container

```yaml
pent_hive_spec: "1.0"
metadata:
  name: attacker
  description: "Kali Linux tools container"
  tags: ["os:kali", "role:attacker"]
type: CONTAINER
container:
  image: kalilinux/kali-rolling:latest
  network_mode: "host"
constraints:
  allow_privileged: true
  max_memory: "2Gi"
```

### Remote Target

```yaml
pent_hive_spec: "1.0"
metadata:
  name: web_server
  description: "Client's web server for testing"
  owner: "client-corp"
  tags: ["os:ubuntu", "role:target"]
type: REMOTE
connection:
  host: "@secret(CLIENT_TARGET_IP)"
  user: "@secret(SSH_USERNAME)"
  port: 2222
  ssh_key_ref: "@secret(CLIENT_SSH_KEY)"
consent_required: true
consent_id: "pentest-approval-2025-09-client-corp"
constraints:
  allow_privileged: false
  allowed_networks:
    - "203.0.113.0/24"
```

### Isolated Test Environment

```yaml
pent_hive_spec: "1.0"
metadata:
  name: isolated_lab
  description: "Isolated container for dangerous tools"
type: CONTAINER
container:
  image: ubuntu:22.04
  network_mode: "none"  # No network access
constraints:
  allow_privileged: false
  max_cpu: "200m"
  max_memory: "512Mi"
```
