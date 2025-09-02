# Commands Reference

This document provides an overview of the commands available in the Pent-Hive CLI. Each command is designed to streamline the setup, validation, and execution of penetration testing workflows.

For more details on the YAML API used by these commands, refer to the [YAML API Overview](overview.md).

## Commands

### `init`
Initializes a new Pent-Hive project by creating the necessary folder structure for bees, hives, and queens.

**Usage**:
```bash
pent-hive init
```

**Details**:
- Creates the following directories:
  - `bees/`: For defining task executors.
  - `hives/`: For specifying execution environments.
  - `queens/`: For organizing and synchronizing tasks.

 ---

### `test`
**Description**: Executes the queens, which in turn run the bees in the specified hives.

**Usage**:
```bash
pent-hive test
```

**Details**:
- Runs the tasks defined in the queen YAML files.
- Ensures proper synchronization between bees and hives during execution.

For more details on defining tasks, see the [Bee Specialization](api/bee_spec.md).

---

### `build`
**Description**: Executes build commands specified by the bees. This is useful for tasks that require building from source.

**Usage**:
```bash
pent-hive build
```

**Details**:
- Looks for build instructions in the bee YAML files.
- Automates the build process for bees that require compilation or setup.

For more details on build instructions, see the [Bee Specialization](api/bee_spec.md).

---

### `lint`
**Description**: Validates the YAML configuration files for bees, hives, and queens to ensure there are no naming mistakes or structural issues.

**Usage**:
```bash
pent-hive lint
```

**Details**:
- Checks for errors in the YAML files.
- Ensures consistency and correctness in the configuration.

For more details on YAML structure, see the [YAML API Overview](overview.md).

---

### `reset`
**Description**: Resets the environment for the specified bees to a clean state, making them ready for re-execution.

**Usage**:
```bash
pent-hive reset
```

**Details**:
- Executes the `reset-steps` defined in the execution section of the bee YAML files.
- If no `reset-steps` are defined, the command does nothing for that bee.

For more details on reset steps, see the [Bee Specialization](api/bee_spec.md).