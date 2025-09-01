# How Pent-Hive Works

Pent-Hive is a system designed to simplify and automate penetration testing workflows. By organizing tasks, environments, and orchestration into structured components, Pent-Hive enables users to focus on testing rather than managing complex setups.

## Core Concepts

### Bees
Bees are the workers of the system, responsible for defining individual tasks or actions. There is no limit to what bees can do: they can execute penetration testing exploits or perform preparatory steps on the target machine to enable such exploits. 

Additionally, bees offer features to expedite workflows:

- They can track dependencies required by the bee. (see [Dependence types in the Bee Specification](api/bee_spec.md#dependency-types) for more information)
- If a bee needs a custom executable, there is a section to provide build instructions, allowing you to compile necessary files before running the bee. (see [Build Section in the Bee Specification](api/bee_spec.md#2-build-section) for more information)
- The execution section can include a reset sub-section, which contains instructions to restore the environment to its pre-bee state. (see [Execution Section feilds in the Bee Specification](api/bee_spec.md#fields) for more information)

> [!note] 
> The build feature draws inspiration from the Unix make tool. The build section acts as a make target, where you can specify build dependencies and steps. Like make, the build process only runs if dependencies change; otherwise, it is skipped since the output is up to date.

### Hives
Hives specify the environments where Bees run. These can include:

- **Host**: The local machine.
- **Remote**: A remote system accessed via SSH.
- **Docker**: A containerized environment.

Hives abstract away the complexity of managing different execution environments.

### Queens
Queens orchestrate the execution of Bees across Hives. They ensure tasks are synchronized and executed in the correct order, making complex workflows manageable.

## Workflow

1. **Initialization**:
	   - Use the `init` command to create the required folder structure.

2. **Configuration**:
	   - Define Bees, Hives, and Queens using YAML files.

3. **Validation**:
	   - Run the `lint` command to check for errors in the YAML files.

4. **Build**:
	   - Use the `build` command to prepare dependencies for Bees.

5. **Execution**:
	   	- Run the `test` command to execute tasks defined in the Queen YAML files.

6. **Reset**:
	   - Use the `reset` command to clean up and prepare the environment for re-execution.

## File Structure

A typical Pent-Hive project follows this structure:

```
project-root/
  bees/
    example. bee.yaml
  hives/
    example.hive.yaml
  queens/
    example.queen.yaml
```

- **`bees/`**: Contains YAML files defining tasks or actions.
- **`hives/`**: Contains YAML files specifying execution environments.
- **`queens/`**: Contains YAML files orchestrating tasks across environments.

## Example Use Case

Imagine a penetration testing lab where you want to test ARP poisoning:
1. Define a Bee to send ARP requests and another to poison the ARP table.
2. Configure Hives for the attacker and target machines.
3. Use a Queen to orchestrate the Bees, ensuring tasks are executed in the correct order.
4. Run the `test` command to execute the workflow.
5. Use the `reset` command to clean up and prepare for the next test.

By following this structured approach, Pent-Hive makes penetration testing workflows repeatable, organized, and efficient.
