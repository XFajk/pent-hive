# Pent-Hive

## 🐝 Hive & Bees

A structured way to organize, automate, and reproduce pentests.

## ✨ The Problem

Pentesting usually means writing and running lots of scripts and tools:

* Build a custom exploit or payload.
* Run sequences of commands against a target.
* Reset/reinitialize the environment to test again.

Doing this manually is:

* **Unorganized** – scattered bash scripts or ad-hoc notes.
* **Error-prone** – easy to forget steps or mess up order.
* **Not portable** – scripts work on *your* machine, but not others.

## 🐝 The Solution

Hive & Bees provides a clean structure:

* **Bees** → units of work.
  Each Bee can define:

  * `build` → compile/setup tools before execution.
  * `run` → execute the actual test/exploit.
  * `reset` → restore the environment to its pre-test state.

* **Hives** → environments where Bees run.
  Abstracts away *where* the pentest happens:

  * Local (host machine).
  * Docker containers.
  * Remote targets.

* **Queens** → orchestrators that tie Bees and Hives together into workflows.

This way, you can keep pentests **organized, reusable, and cross-platform**.

## 🚀 Example

### A simple Bee (`bees/sql-injection.yaml`)

```yaml
description: Test for SQL Injection
build-steps:
  - pip install -r requirements.txt
steps:
  - python exploit.py --url http://target/login
reset-steps:
  - rm exploit.log
hives:
  - docker-local
```

### A simple Hive (`hives/docker-local.yaml`)

```yaml
type: docker
image: vulnerables/web-dvwa
ports:
  - "8080:80"
```

### Running it with a Queen

```bash
queen run bees/sql-injection.yaml --hive hives/docker-local.yaml
```

This will:

1. Spin up the DVWA container.
2. Build the exploit requirements.
3. Run the exploit.
4. Reset the environment.

## 🎯 Why Hive & Bees?

* Keep pentests organized in one place.
* Automate repetitive build/run/reset cycles.
* Reproduce results easily across machines.
* Scale from local Docker labs → to host machines → to real remote targets.

