# Jack-Do 🐦

A developer productivity CLI tool that I'm building as part of my journey to learn and master Rust. It's a place where I experiment with performance, memory safety, and modern CLI design.

## Features

### TypeScript

#### `remove-unused-declarations`

Identifies and removes unused variables, functions, classes, and imports from your TypeScript files.

- **Smart Destructuring**: Intelligently removes only the unused parts of a destructuring assignment.
- **Multi-line Support**: Correctly handles and cleans up code spanning multiple lines.
- **Safe**: Automatically preserves exported symbols.

```bash
jack-do typescript remove-unused-declarations "src/**/*.ts"
```

## Installation

`jack-do` provides robust installation scripts that automate building, directory setup, and PATH configuration while checking for necessary dependencies.

### Windows (PowerShell)

Run the following command to install `jack-do` to `$HOME\.jack-do\bin`:

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\install.ps1
```

### Unix-like (Linux/macOS)

Run the following script to install `jack-do` to `$HOME/.jack-do/bin`:

```bash
chmod +x ./scripts/install.sh
./scripts/install.sh
```

> [!TIP]
> Both scripts include built-in recovery: if the installation fails, they will clean up partially installed files to leave your system in a clean state.

## Development

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable)

### Building from source

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

## License

MIT

---

### Design & Internal Architecture

Deep dive into how the tool is built:

- **[Design Abstraction](docs/architecture/design_abstraction.md)**: High-level patterns.
- **[Technical Specifications](docs/architecture/technical_specs.md)**: Dependencies and engine mechanics.
- **[Thought Process](docs/architecture/thought_process.md)**: Why I made these choices.

### Learning Rust?

> [!IMPORTANT]
> **Personal Learning Notes**: The documents below contain my personal findings while learning Rust. It is not an official Rust guide or standard learning material. For official documentation, please visit [doc.rust-lang.org](https://doc.rust-lang.org).

Moving from a TypeScript background? I've got you covered:

- **[My TypeScript to Rust Journey](docs/learning/ts_to_rust_anecdotes.md)**: Conceptual comparisons and relatable anecdotes.
- **[My Rust Learning Notes: Concepts in Jack-Do](docs/learning/rust_concepts.md)**: Deep dive into Ownership, Lifetimes, and more.
- **[Why Rust?](docs/learning/ts_vs_rust_why.md)**: Why Rust is better for high-performance applications.
