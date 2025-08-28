# YAML API Overview

This section defines the YAML API contract for Bees, Hives, and Queens. Write these documents first to lock down the data shapes before implementing the runtime.

Design goals:

- Explicit, simple fields with typed values.
- Validation via `queen validate`.
- Avoid embedding secrets in YAML; use environment variables or external vaults.

The following pages describe each resource in detail.
