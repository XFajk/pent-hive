# Pent-Hive AI Assistant Context

This document provides comprehensive context for AI assistants working on the Pent-Hive project.

## Project Overview

**Pent-Hive** is a structured penetration testing automation tool built in Rust. It organizes pentests into three YAML-based components that work together to create reproducible, safe, and auditable security testing workflows.

### Core Architecture

1. **Bees** (`*.bee.yaml`) - Individual tasks/commands (the "what")
2. **Hives** (`*.hive.yaml`) - Execution environments (the "where") 
3. **Queens** (`*.queen.yaml`) - Orchestration workflows (the "how")

## Design Philosophy

### Documentation-Driven Development
- Documentation was written BEFORE implementation
- Schemas define the contracts and serve as implementation blueprints
- User experience was designed first, implementation follows
- All examples in docs must work when implemented

### Security-First Approach
- No hardcoded secrets in YAML files
- Use `@secret(KEY)` tokens that resolve from `.env` files
- Secrets are masked in `--dry-run` output
- Explicit consent required for remote targets
- Network operations require `--allow-network` flag

### Safety & Auditability
- `--dry-run` is mandatory before real execution
- All actions are logged and traceable
- Dependency validation prevents broken workflows
- Consent tracking for compliance

## File Structure & Schemas

### Required Schema Version
All YAML files must start with:
```yaml
pent_hive_spec: "1.0"
```

### Bees (Tasks)
**File naming**: `*.bee.yaml`
**Purpose**: Define individual tasks/commands

**Required fields**:
- `pent_hive_spec: "1.0"`
- `metadata.name` (string)
- `execution.steps` (array, min 1 item)

**Key concepts**:
- `build` section for tool compilation/preparation
- `execution` section for runtime commands
- Parameters via `@1`, `@2` (positional) or `@name` (named)
- Secrets via `@secret(KEY)` tokens
- Dependencies specify requirements (CMD, FILE, DIR, HIVE, SHELL)
- Artifacts collect output files
- Reset steps for cleanup

### Hives (Environments)
**File naming**: `*.hive.yaml`
**Purpose**: Define where Bees execute

**Required fields**:
- `pent_hive_spec: "1.0"`
- `metadata.name` (string)
- `type` (enum: HOST | REMOTE | CONTAINER)

**Three types**:
1. **HOST** - Local machine execution
2. **REMOTE** - SSH to remote machine (requires `connection` object)
3. **CONTAINER** - Docker container (requires `container` object)

**Key concepts**:
- `constraints` control privileges and resource limits
- `consent_required` + `consent_id` for remote targets
- SSH connections use `@secret()` for credentials
- Network isolation via `allowed_networks` CIDR lists

### Queens (Orchestration)
**File naming**: `*.queen.yaml`
**Purpose**: Coordinate Bees across Hives

**Required fields**:
- `pent_hive_spec: "1.0"`
- `metadata.name` (string)
- `steps` (array, min 1 item)

**Step structure**:
- `id` - unique identifier
- `hive` - target hive name
- `run` - array of Bee invocations
- `depends_on` - array of step IDs (creates DAG)
- `parallel` - concurrent execution flag
- `on_failure` - failure policy (fail_fast|continue|mark_and_continue)

**Key concepts**:
- DAG-based execution with dependency resolution
- Parameter passing to Bees via `run` array
- Assertions map exit codes to outcomes
- Global timeout and per-step timeouts

## Secret Management System

### `.env` File Format
```bash
# Sensitive configuration
TARGET_IP=192.168.1.100
SSH_USERNAME=pentester
SSH_PRIVATE_KEY=/path/to/key
API_TOKEN=sk-1234567890abcdef
WORDLIST_PATH=/usr/share/wordlists/rockyou.txt
```

### Token Usage
- **In YAML**: Use `@secret(KEY_NAME)` 
- **Runtime**: Resolved from environment variables or `.env` file
- **Dry-run**: Shows as `[MASKED]` for security
- **Precedence**: Environment variables override `.env` file

### Security Features
- Never store plaintext secrets in YAML
- Secrets loaded at runtime only
- Masked in all logging and dry-run output
- Linter detects hardcoded secret patterns

## CLI Commands

### Primary Workflow
```bash
pent-hive init --with-schemas --with-vscode  # Setup project
pent-hive lint                               # Validate YAML
pent-hive test workflow --dry-run           # Preview execution  
pent-hive test workflow --allow-network     # Execute for real
```

### Command Details
- `init` - Create project structure, optional schema/editor setup
- `lint` - Schema validation + cross-file checks + secret scanning
- `build` - Execute build steps from Bees (cached)
- `test` - Run Queen workflows with safety checks

### Safety Flags
- `--dry-run` - Show execution plan, mask secrets, no actual execution
- `--allow-network` - Required for network operations
- `--consent <id>` - Required for remote targets with consent_required=true

## Implementation Notes

### Technology Stack
- **Language**: Rust
- **Config Format**: YAML with JSON Schema validation
- **Secret Storage**: Environment variables / `.env` files
- **Container Runtime**: Docker integration
- **SSH**: For remote execution

### Validation Layers
1. **JSON Schema** - Structural validation
2. **Cross-file validation** - References between Bees/Hives/Queens
3. **DAG validation** - Cycle detection, dependency resolution
4. **Secret scanning** - Detect hardcoded credentials
5. **Runtime validation** - Consent, network permissions

### Parameter Resolution
```
Queen invocation: ["my_bee target=@secret(HOST) ports=80,443"]
↓
Bee receives: @target="192.168.1.100", @ports="80,443"
↓  
Command executed: "nmap -p 80,443 192.168.1.100"
```

## Documentation Standards

### Writing Style
- **Human-readable** - Avoid jargon, explain clearly
- **Example-driven** - Show don't tell with working YAML
- **Security-focused** - Always show `@secret()` usage
- **Practical** - Users should be able to copy/paste examples

### Structure Requirements
- No repetition across documents
- Each concept explained once in the right place
- Logical progression: concepts → CLI → detailed specs
- Minimal use of callout boxes (only for important warnings/tips)

### Example Requirements
- All YAML examples must be valid against schemas
- Show realistic pentest scenarios, not toy examples
- Always use `@secret()` for sensitive data
- Include both simple and complex use cases

## Common Patterns

### Basic Bee Structure
```yaml
pent_hive_spec: "1.0"
metadata:
  name: task_name
  params:
    named: ["target", "ports"]
execution:
  dependencies:
    - type: CMD
      value: nmap
  steps:
    - run: "nmap -p @ports @target"
  artifacts:
    - "results.txt"
```

### Remote Hive with Secrets
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
consent_id: "pentest-approval-2025-client"
```

### Queen with Dependencies
```yaml
pent_hive_spec: "1.0"
metadata:
  name: recon_workflow
steps:
  - id: discovery
    hive: attacker
    run: ["host_discovery @secret(TARGET_NETWORK)"]
    
  - id: port_scan
    hive: attacker
    depends_on: ["discovery"]
    run: ["port_scan @secret(TARGET_IP)"]
    
  - id: report
    hive: local
    depends_on: ["port_scan"]
    run: ["generate_report client=@secret(CLIENT_NAME)"]
```

## Key Design Decisions

### Why YAML over JSON/TOML?
- Human-readable for pentesters
- Comments support for documentation
- Multi-line strings for complex commands
- Industry standard for configuration

### Why Three-Tier Architecture?
- **Separation of concerns**: What vs Where vs How
- **Reusability**: Same Bee can run on different Hives
- **Safety**: Explicit environment declarations
- **Flexibility**: Complex orchestration without complexity in tasks

### Why Schema-First?
- Documentation as implementation contract
- Early validation of design decisions
- Better user experience through upfront design
- Easier to maintain consistency

### Why Secret Tokens?
- Security: No plaintext credentials in version control
- Flexibility: Same YAML works across environments
- Auditability: Clear distinction between config and secrets
- Safety: Automatic masking in dry-run output

## When Extending the System

### Adding New Features
1. Update schemas first
2. Update documentation with examples
3. Ensure examples work in dry-run mode
4. Implement with schema validation
5. Add linting rules if needed

### Maintaining Backward Compatibility
- Schema version field allows evolution
- Deprecate features gradually
- Provide migration tools/docs
- Keep old examples working

### Security Considerations
- Never add features that could leak secrets
- Always require explicit opt-ins for dangerous operations
- Maintain consent audit trail
- Validate all user inputs

This context should enable any AI assistant to understand the project's goals, architecture, and implementation requirements without needing to read through all the documentation.
