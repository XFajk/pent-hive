# Bee Specification

A Bee is a YAML file that defines a single task or command. Bees are the building blocks of your pentest workflows.

## File Structure

Bee files must:

- End with `.bee.yaml`
- Start with `pent_hive_spec: "1.0"`
- Have a unique `metadata.name`

## Required Fields

```yaml
pent_hive_spec: "1.0"
metadata:
  name: my_task
execution:
  steps:
    - run: "echo Hello"
```

## Full Structure

```yaml
pent_hive_spec: "1.0"
metadata:
  name: port_scanner
  description: "Scans ports on target"
  tags: ["network", "recon"]
  params:
    named: ["target", "ports"]
    defaults:
      ports: "1-1000"

build:
  dependencies:
    - type: CMD
      value: gcc
  steps:
    - name: "compile scanner"
      run: "gcc scanner.c -o scanner"

execution:
  dependencies:
    - type: CMD
      value: nmap
  steps:
    - name: "scan ports"
      run: "nmap -p @ports @target"
      timeout_s: 300
  artifacts:
    - "scan_results.txt"
  env:
    SCAN_OUTPUT: "/tmp/scan.log"
  privileged: false
```

## Metadata Section

### Required

- `name` - Unique identifier for this Bee

### Optional

- `description` - Human-readable description
- `tags` - Array of labels for organization
- `params` - Parameter definitions
  - `named` - List of named parameters this Bee accepts
  - `defaults` - Default values for parameters

## Build Section (Optional)

Used to prepare tools before execution:

- `dependencies` - What's needed to build
- `steps` - Commands to run

## Execution Section (Required)

Defines what the Bee actually does:

- `dependencies` - Runtime requirements
- `steps` - Commands to execute (at least one required)
- `reset_steps` - Cleanup commands
- `artifacts` - Files to collect after execution
- `env` - Environment variables
- `privileged` - Whether to run with elevated privileges
- `retry_policy` - How to handle failures

## Parameters

Bees can accept parameters using `@` tokens:

- `@1`, `@2`, etc. - Positional parameters
- `@name` - Named parameters
- `@secret(KEY)` - Secrets from environment variables

Example:

```yaml
# Queen passes parameters
run: ["auth_scanner 192.168.1.1 username=admin"]

# Bee receives them
run: "hydra -l @username -P @secret(PASSWORD_LIST) @1 ssh"
```

### Working with Secrets

Secrets are loaded from environment variables or `.env` files:

```bash
# .env file
API_KEY=sk-1234567890abcdef
DB_PASSWORD=super_secret_password
SSH_PRIVATE_KEY=/home/user/.ssh/id_rsa
```

```yaml
# Bee using secrets
execution:
  steps:
    - run: "curl -H 'Authorization: Bearer @secret(API_KEY)' @target_url"
  env:
    DB_PASS: "@secret(DB_PASSWORD)"
```

## Dependencies

Dependencies specify what must be available:

```yaml
dependencies:
  - type: CMD
    value: nmap        # Command must exist
  - type: FILE
    value: /etc/hosts  # File must exist
  - type: DIR
    value: /tmp        # Directory must exist
```

Types: `CMD`, `FILE`, `DIR`, `HIVE`, `SHELL`

## Steps

Steps define the actual work:

```yaml
steps:
  - run: "simple command"
  - name: "complex step"
    run: ["multiple", "commands"]
    shell: bash
    timeout_s: 60
    env:
      CUSTOM_VAR: "value"
```

Each step can have:

- `name` - Description
- `run` - Command(s) to execute
- `shell` - Shell to use
- `timeout_s` - Time limit
- `env` - Environment variables

> [!NOTE]
> Steps run in order. If one fails, the Bee stops unless a retry policy is defined.

## Examples

### Simple Command Bee

```yaml
pent_hive_spec: "1.0"
metadata:
  name: ping_host
execution:
  steps:
    - run: "ping -c 4 @1"
```

### Tool Building Bee

```yaml
pent_hive_spec: "1.0"
metadata:
  name: custom_scanner
build:
  steps:
    - run: "gcc scanner.c -o scanner"
execution:
  dependencies:
    - type: FILE
      value: "./scanner"
  steps:
    - run: "./scanner @target"
  artifacts:
    - "results.json"
```

### Authentication Scanner

```yaml
pent_hive_spec: "1.0"
metadata:
  name: brute_force
  params:
    named: ["target", "username"]
execution:
  steps:
    - name: "brute force login"
      run: "hydra -l @username -P @secret(WORDLIST_PATH) @target ssh"
      timeout_s: 600
  env:
    HYDRA_OPTS: "@secret(HYDRA_OPTIONS)"
```

### API Testing Bee

```yaml
pent_hive_spec: "1.0"
metadata:
  name: api_test
  params:
    named: ["endpoint"]
execution:
  steps:
    - name: "authenticated API call"
      run: "curl -H 'Authorization: Bearer @secret(API_TOKEN)' @endpoint"
    - name: "save response"
      run: "echo $RESPONSE > api_results.json"
  artifacts:
    - "api_results.json"
```
