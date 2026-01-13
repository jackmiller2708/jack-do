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
- **[Design Abstraction](docs/design_abstraction.md)**: High-level patterns.
- **[Technical Specifications](docs/technical_specs.md)**: Dependencies and engine mechanics.
- **[Thought Process](docs/thought_process.md)**: Why I made these choices.

### Learning Rust?
Moving from a TypeScript background? I've got you covered:
- **[TS to Rust Anecdotes & Comparisons](docs/ts_to_rust_anecdotes.md)**: Conceptual comparisons and relatable anecdotes.
- **[Technical Rust Concepts Guide](docs/rust_concepts.md)**: Deep dive into Ownership, Lifetimes, and more.
