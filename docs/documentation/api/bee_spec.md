# Bee Specification

Bees are the backbone of the Pent-Hive system. They define the tasks or actions to be executed on hives, such as running commands, building programs, or preparing environments. Bees are designed to be flexible and powerful, enabling users to automate complex workflows.

## Naming Convention

- The name of a bee is derived from its filename. For example, a file named `arp_poisoning.bee.yaml` defines a bee named `arp_poisoning`.
- All bee files must include the `.bee.yaml` extension to be recognized as valid bee definitions.

## Structure of a Bee YAML File

A bee YAML file consists of two main sections: **Build** and **Execution**. Each section can define dependencies and steps.
### 1. Execution Section

The execution section defines the main task or action the bee performs.

#### Fields:
- `dependencies`: A list of dependencies required for the execution process.
- `steps`: A list of shell commands to execute during the bee's runtime.
- `reset-steps` (optional): A list of shell commands to reset the environment to a clean state.

#### Example:
```yaml
execution:
  - dependencies:
    - HIVE target
    - CMD python3
    - FILE ./config.json
    - SHELL bash
  - steps:
    - python3 exploit.py --config ./config.json
  - reset-steps:
    - rm ./output.log
    - pkill exploit
```

### 2. Build Section

The build section is optional and is used to prepare the environment or build tools required for the bee's execution.

#### Fields:
- `dependencies`: A list of dependencies required for the build process.
- `steps`: A list of shell commands to execute during the build process.

#### Example:
```yaml
build:
  - dependencies:
    - CMD cmake
    - FILE ./CMakeLists.txt
  - steps:
    - cmake .
    - make
```

## Dependency Types

Dependencies ensure that the required conditions are met before a bee can run. There are five types of dependencies:

1. **HIVE**: Specifies the hives where the bee can run. Multiple hive dependencies mean the bee can run on any of the specified hives.
	   - Example: `- HIVE target`

2. **CMD**: Ensures that a specific command is available in the hive's environment.
	- Example: `- CMD cmake`

3. **FILE**: Ensures that a specific file exists in the hive's environment.
	   - Example: `- FILE ./config.json`

4. **DIR**: Ensures that a specific directory exists in the hive's environment.
	   - Example: `- DIR ./src/`

5. **SHELL**: Specifies the shells in which the bee can run. If no shell is specified, the bee can run in any shell.
	   - Example: `- SHELL bash`
	   - Example: `- SHELL sh`

## Steps

Steps are shell commands executed in the hive's environment. They can be any valid command supported by the hive's shell, including scripts, compiled binaries, or even Python code.