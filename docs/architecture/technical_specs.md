# Technical Specifications

`jack-do` is built for extreme performance and correctness. Here are the technical specs of the engine I'm building.

## Core Dependencies

I chose these tools because they represent the "gold standard" in the Rust ecosystem for CLI development:

| Library       | Role                             | Why I chose it                                                                                                                   |
| :------------ | :------------------------------- | :------------------------------------------------------------------------------------------------------------------------------- |
| **Clap (v4)** | Command Line Parser              | The most robust and feature-rich CLI parser in Rust. Supports the domain-based structure perfectly.                              |
| **OXC**       | JS/TS Parser & Semantic Analysis | **The fastest JS parser** written in Rust. It's much faster than SWC or Biome and provides deep semantic data (symbols, scopes). |
| **Tokio**     | Async Runtime                    | Allows `jack-do` to process multiple files in parallel without blocking, significantly speeding up large codebase scans.         |
| **Anyhow**    | Error Management                 | Simplifies error handling with context, making it easier to track why a specific file operation failed.                          |
| **Tracing**   | Diagnostics                      | Structured logging that allows me to see exactly what the tool is doing under the hood during a run.                             |

## The Transformation Engine (TypeScript)

The TypeScript domain doesn't just "regex" its way through files. It uses a full compiler pipeline:

### 1. Parsing

I use the `oxc_parser` to turn source code into an AST.

- **Spec**: ECMAScript 2024 + TypeScript 5.x support.
- **Speed**: Capable of parsing millions of lines of code per second.

### 2. Semantic Analysis

Unlike simpler tools, `jack-do` builds a full "Semantic Model" using `oxc_semantic`.

- **Scope Tree**: Understands nested functions and blocks.
- **Symbol Table**: Tracks every variable declaration and its usages.
- **Reference Checking**: I check if a symbol is "referenced" or "exported" before marking it for removal.

### 3. Span Manipulation

Instead of re-printing the entire AST (which often loses original formatting/comments), I use **Span Deletion**.

- I identify the exact `[start, end]` byte offsets of unused code.
- I remove these spans from the original source string.
- This preserves the user's styling, comments, and whitespace in the rest of the file.

## Performance Considerations

- **Memory Safety**: By leveraging Rust's ownership model, `jack-do` processes files with zero risk of memory leaks or data races.
- **Parallelism**: I use `tokio` to run file operations concurrently. Large projects with 1,000+ files are processed in a fraction of a second.
- **Zero-Copy**: Whenever possible, I work with references (`&str`) to the original file content to avoid unnecessary memory allocations.
