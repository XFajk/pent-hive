# About

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

### **Bees** → units of work.

  Each Bee can define:

* `build` → compile/setup tools before execution.
* `run` → execute the actual test/exploit.
* `reset` → restore the environment to its pre-test state.

### **Hives** → environments where Bees run.

  Abstracts away *where* the pentest happens:

* Local (host machine).
* Docker containers.
* Remote targets.

### **Queens** → orchestrators that tie Bees and Hives together into workflows.

This way, you can keep pentests **organized, reusable, and cross-platform**.
