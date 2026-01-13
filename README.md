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

### Windows (Quick Install)
Run the following command in PowerShell to build and install `jack-do` to your system's PATH:

```powershell
.\scripts\install.ps1
```

### Manual Installation

1. **Build the project**:
   ```bash
   cargo build --release
   ```
2. **Add to PATH**:
   Add the `target/release` directory (or copy the `jack-do.exe` binary to a folder in your PATH).

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

### Learning Rust? 🦀
Moving from a TypeScript background? I've got you covered:
- **[TS to Rust Anecdotes & Comparisons](docs/ts_to_rust_anecdotes.md)**: Conceptual comparisons and relatable anecdotes.
- **[Technical Rust Concepts Guide](docs/rust_concepts.md)**: Deep dive into Ownership, Lifetimes, and more.
