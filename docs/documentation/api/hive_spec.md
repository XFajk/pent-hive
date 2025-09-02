# Hive Specification

Hives define the environments where Bees execute their tasks. They abstract away the complexity of managing different execution environments, allowing users to focus on defining tasks and workflows.

## Naming Convention

- The name of a Hive is derived from its filename. For example, a file named `target.hive.yaml` defines a Hive named `target`.
- All Hive files must include the `.hive.yaml` extension to be recognized as valid Hive definitions.

## Structure of a Hive YAML File

A Hive YAML file consists of a `type` field and additional fields depending on the type of Hive.

### 1. Type Field

The `type` field specifies the type of Hive. It can have one of the following values:

- `HOST`: Runs tasks on the local machine.
- `REMOTE`: Runs tasks on a remote machine via SSH.
- `CONTAINER`: Runs tasks inside a Docker container.

### 2. Fields by Type

#### HOST
The `HOST` type runs bees on the local machine. It does not require any additional fields.

##### Example:
```yaml
type: HOST
```

#### REMOTE
The `REMOTE` type runs bees on a remote machine via SSH. It requires the following fields:

- `user`: The username for SSH access.
- `url`: The URL or IP address of the remote machine.
- `port`: The port for SSH access (default is 22).
- `flags` (optional): Additional flags to pass to the SSH command.

**Note**: Handling passwords securely is critical. It is recommended to use SSH keys or an external vault for password management.

##### Example:
```yaml
type: REMOTE
user: pentester
url: 192.168.1.100
port: 22
flags: "-o StrictHostKeyChecking=no"
```

#### CONTAINER
The `CONTAINER` type runs tasks inside a Docker container. It requires the following fields:

- `image`: The Docker image to use.
- `flags` (optional): Additional flags to pass to the Docker command.

##### Example:
```yaml
type: CONTAINER
image: ubuntu:latest
flags: "--rm"
```

## Validation Rules

- The `type` field is required and must be one of `HOST`, `REMOTE`, or `CONTAINER`.
- Additional fields must match the requirements for the specified type.

## Examples

### HOST Hive
```yaml
type: HOST
```

### REMOTE Hive
```yaml
type: REMOTE
user: admin
url: 10.0.0.1
port: 2222
flags: "-o StrictHostKeyChecking=no"
```

### CONTAINER Hive
```yaml
type: CONTAINER
image: alpine:latest
flags: "--rm -it"
```