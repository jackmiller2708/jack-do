# Jack-Do 🐦

A powerful developer productivity CLI tool built for speed and precision.

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
