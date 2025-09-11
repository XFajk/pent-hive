# Queen Specification

A Queen orchestrates Bees across Hives to create workflows. Queens define the order, dependencies, and coordination of your pentest steps.

## File Structure

Queen files must:

- End with `.queen.yaml`
- Start with `pent_hive_spec: "1.0"`
- Have a unique `metadata.name`

## Required Fields

```yaml
pent_hive_spec: "1.0"
metadata:
  name: my_workflow
steps:
  - id: step1
    hive: my_hive
    run: ["my_bee"]
```

## Full Structure

```yaml
pent_hive_spec: "1.0"
metadata:
  name: network_recon
  description: "Complete network reconnaissance workflow"
  tags: ["recon", "network"]

steps:
  - id: port_scan
    hive: attacker
    run: ["nmap_scan @secret(TARGET_IP)"]
    timeout_s: 300

  - id: service_enum
    hive: attacker
    depends_on: ["port_scan"]
    run: 
      - "service_detect @secret(TARGET_IP)"
      - "vulnerability_scan @secret(TARGET_IP) depth=full"
    parallel: true

  - id: report
    hive: local
    depends_on: ["service_enum"]
    run: ["generate_report client=@secret(CLIENT_NAME)"]

assertions:
  - type: exit_code_map
    mapping:
      "0": "COMPLETE"
      "nonzero": "FAILED"
```

## Metadata Section

### Required
- `name` - Unique identifier for this Queen

### Optional
- `description` - Human-readable description
- `tags` - Array of labels for organization

## Steps - The Heart of Workflows

Each step defines work to be done:

```yaml
steps:
  - id: unique_step_name
    hive: target_hive_name
    run: ["bee_name param1 param2"]
```

### Required Fields
- `id` - Unique identifier within this Queen
- `hive` - Name of Hive where this step runs
- `run` - Array of Bee invocations

### Optional Fields
- `depends_on` - Array of step IDs this step waits for
- `parallel` - If true, run Bees in this step concurrently
- `timeout_s` - Time limit for the entire step
- `on_failure` - What to do if step fails: `fail_fast` (default), `continue`, `mark_and_continue`

## Dependencies & Ordering

Steps run based on their dependencies:

```yaml
steps:
  - id: scan_ports
    hive: attacker
    run: ["port_scanner @target"]

  - id: scan_vulns
    hive: attacker
    depends_on: ["scan_ports"]  # Waits for port scan
    run: ["vuln_scanner @target"]

  - id: generate_report
    hive: local
    depends_on: ["scan_ports", "scan_vulns"]  # Waits for both
    run: ["report_generator"]
```

## Calling Bees with Parameters

Pass parameters to Bees in the `run` array:

```yaml
# Positional parameters
run: ["my_bee 192.168.1.1 80,443"]

# Named parameters  
run: ["my_bee target=192.168.1.1 ports=80,443"]

# Mixed parameters with secrets
run: ["auth_test @secret(TARGET_IP) username=admin password=@secret(TEST_PASSWORD)"]

# Multiple Bees with different parameters
run: 
  - "port_scan @secret(TARGET_IP)"
  - "service_scan @secret(TARGET_IP) aggressive=true"
  - "auth_test @secret(TARGET_IP) user=@secret(USERNAME)"
```

The Bee receives these as `@1`, `@target`, `@username`, etc.

## Working with Secrets in Queens

Queens can pass secrets to Bees:

```yaml
steps:
  - id: authenticated_scan
    hive: attacker
    run: 
      - "login_test target=@secret(TARGET_HOST) user=@secret(SSH_USER)"
      - "api_test endpoint=@secret(API_ENDPOINT) token=@secret(API_KEY)"
```

## Parallel Execution

Control parallelism at two levels:

### Step-Level Parallelism
Steps without dependencies can run simultaneously:

```yaml
steps:
  - id: scan_tcp
    hive: attacker
    run: ["tcp_scanner @target"]
    
  - id: scan_udp  # Runs at same time as tcp_scanner
    hive: attacker  
    run: ["udp_scanner @target"]
```

### Bee-Level Parallelism
Multiple Bees in one step can run concurrently:

```yaml
steps:
  - id: multi_scan
    hive: attacker
    parallel: true
    run:
      - "port_scan @target"
      - "service_scan @target"
      - "os_detection @target"
```

## Failure Handling

Control what happens when steps fail:

```yaml
steps:
  - id: optional_step
    hive: attacker
    run: ["might_fail"]
    on_failure: continue  # Don't stop the workflow
    
  - id: critical_step
    hive: attacker
    run: ["must_succeed"]
    on_failure: fail_fast  # Stop immediately (default)
```

Options:
- `fail_fast` - Stop the entire workflow (default)
- `continue` - Continue to next steps
- `mark_and_continue` - Mark as failed but continue

## Assertions

Map step outcomes to higher-level results:

```yaml
assertions:
  - type: exit_code_map
    mapping:
      "0": "SECURE"
      "1": "VULNERABLE" 
      "nonzero": "FAILED_TO_EXECUTE"
```

## Global Settings

### Timeout
Set a global timeout for the entire workflow:

```yaml
timeout_s: 3600  # 1 hour max
```

## Examples

### Simple Linear Workflow
```yaml
pent_hive_spec: "1.0"
metadata:
  name: basic_scan
steps:
  - id: recon
    hive: attacker
    run: ["nmap_scan @target"]
    
  - id: report
    hive: local
    depends_on: ["recon"]
    run: ["generate_report"]
```

### Complex Parallel Workflow
```yaml
pent_hive_spec: "1.0"
metadata:
  name: full_assessment
  description: "Complete security assessment"

steps:
  # Initial reconnaissance
  - id: discovery
    hive: attacker
    run: ["host_discovery 10.0.0.0/24"]

  # Parallel scanning phase
  - id: port_scan
    hive: attacker
    depends_on: ["discovery"]
    run: ["port_scanner @discovered_hosts"]

  - id: service_scan  
    hive: attacker
    depends_on: ["port_scan"]
    parallel: true
    run:
      - "service_detection @open_ports"
      - "ssl_scanner @https_services"
      - "web_scanner @web_services"

  # Exploitation attempts
  - id: exploit_attempts
    hive: attacker
    depends_on: ["service_scan"]
    run: ["exploit_framework @vulnerable_services"]
    on_failure: continue  # Don't fail if exploits don't work

  # Documentation
  - id: documentation
    hive: local
    depends_on: ["exploit_attempts"]
    run: ["generate_report", "create_timeline"]

assertions:
  - type: exit_code_map
    mapping:
      "0": "ASSESSMENT_COMPLETE"
      "nonzero": "ASSESSMENT_FAILED"

timeout_s: 7200  # 2 hours
```

### Conditional Workflow
```yaml
pent_hive_spec: "1.0"
metadata:
  name: adaptive_scan

steps:
  - id: quick_scan
    hive: attacker
    run: ["fast_port_scan @target"]

  - id: detailed_scan
    hive: attacker
    depends_on: ["quick_scan"]
    run: ["detailed_scan @target"]
    # Only runs if quick_scan found open ports

  - id: cleanup
    hive: attacker
    depends_on: ["detailed_scan"]
    run: ["cleanup_logs"]
    on_failure: continue  # Always try to clean up
```
