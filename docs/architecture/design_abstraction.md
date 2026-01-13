# CLI Design Abstraction

The architecture of `jack-do` is built around a **Domain-Centric Command Pattern**. This design ensures that as I add support for more languages or tools (domains), the codebase remains organized and predictable.

## The Layered Architecture

I've organized the CLI into three distinct layers of responsibility:

```text
+---------------------------------------+
|            ORCHESTRATION LAYER        |
|               (main.rs)               |
|  - Inits logging & error handling     |
|  - Routes to the correct domain       |
+-------------------+-------------------+
                    |
                    v
+-------------------+-------------------+
|             COMMAND LAYER             |
|              (src/cli/*)              |
|  - Defines domains & subcommands      |
|  - Parses flags and arguments         |
|  - Domain-specific CLI definitions    |
+-------------------+-------------------+
                    |
                    v
+-------------------+-------------------+
|             DOMAIN LAYER              |
|           (src/[domain]/*)            |
|  - Encapsulated business logic        |
|  - Analysis (analyzer.rs)             |
|  - Transformation (modifier.rs)       |
+---------------------------------------+
```

## Domain-Command Pattern

The CLI follows a strict hierarchy: `jack-do <domain> <command> <glob>`.

### Why this abstraction?

1. **Discoverability**: Running `jack-do typescript --help` only shows TypeScript-specific commands.
2. **Isolation**: Logic for `typescript` lives in `src/typescript/`, completely isolated from other future domains.
3. **Consistency**: Every domain follows the same `mod.rs` (entry) + submodules pattern.

```text
jack-do (CLI Binary)
 ├── Typescript (Domain)
 │    ├── Remove Unused Declarations (Command)
 │    └── Formatter (Future Command)
 └── Rust (Future Domain)
      ├── Audit (Future Command)
      └── ...
```

## Implementation Strategy: The "Transform" Loop

Most commands in `jack-do` (especially in the TypeScript domain) follow a standard transformation lifecycle:

1. **Discovery**: Use glob patterns to find target files.
2. **Analysis**: Parse the file into an AST (Abstract Syntax Tree).
3. **Identification**: Identify spans of code that need modification (e.g., unused symbols).
4. **Correction**: Apply deletions or updates to the source text based on identified spans.
5. **Persistence**: Write the modified text back to the disk (handled by `modifier.rs`).

This lifecycle is decomposed into dedicated modules like `analyzer.rs` and `modifier.rs`, ensuring that each part of the tool has a single, clear responsibility.
