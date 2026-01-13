# Thought Process & Design Decisions

Building `jack-do` isn't just about the "how," but the "why." This document records the reasoning behind my design choices as I learn Rust.

## Why a Domain-Led CLI?

Most CLI tools are either "one-trick ponies" (do one thing well) or "monoliths" (do everything in one big pile). I wanted `jack-do` to be a **Toolbox**.

- **My Goal**: I wanted a single binary that could help me with TypeScript today, but maybe Rust or CSS tomorrow.
- **The Decision**: By using the `Domain -> Command` structure, I've created a "plug-and-play" architecture. Adding a new domain is as simple as adding a folder and a few lines to `cli.rs`.

## Why OXC over SWC or Biome?

When I started research for the TypeScript domain, I looked at the big players.

- **SWC** is the industry standard for transpilation, but its AST can be complex to work with for simple analysis.
- **OXC** (The Oxidation Compiler) is a newer project focused on performance and _correctness_.
- **My Experience**: OXC's "Semantic Model" (which automatically links variables to their usages) felt more intuitive for a "remove unused code" feature. It felt like OXC was designed for _tools like mine_, not just for bundling.

## The Choice of "Span Removal" vs. "AST Printing"

A big decision I faced was how to save the modified code.

1. **AST Printing**: Turn the AST back into code. This is easy but usually ruins the user's formatting and deletes comments.
2. **Span Removal**: Leave the source code as-is and just "cut out" the bad parts.

**I chose Span Removal.** Even though it's harder to get the commas and line breaks right, it respects the developer's original code. I believe a good tool should be invisible; it should fix the problem without leaving a mess of reformatted code behind.

## The Domain-Driven Refactor: Split by Responsibility

As the TypeScript logic grew, I realized that a single `typescript.rs` file was becoming a "monolith." I decided to refactor it into a domain-driven structure.

- **The Principle**: I shifted from "layer-based" thinking to "domain-based" thinking. Instead of having a generic `domains/` folder, each domain is now its own package-like directory (`src/typescript/`).
- **Cohesion over Layering**: Inside the domain, I split logic by responsibility. AST analysis (the "brain") moved to `analyzer.rs`, while file modification (the "hands") moved to `modifier.rs`.
- **Minimized Surface Area**: I made heavy use of `pub(crate)` to ensure that internal helper functions and structs aren't exposed unnecessarily. This makes the codebase much easier to reason about—if it's not `pub`, it can't leak.

## Learning from Rust's Strictness

Many of the design decisions in the `Logic Layer` were actually driven by the Rust compiler.

- **Result-Driven Logic**: In TypeScript, I might have used a lot of `try/catch`. In Rust, I've embraced returning `Result`. This forced me to think about edge cases (like "what if the file is deleted mid-scan?") before they even happened.
- **The Power of Enums**: Using `match` on the CLI domains and commands makes the routing logic indestructible. If I add a command but forget to handle it in `main.rs`, the code won't compile. This level of safety is addictive!

## Final Philosophy

My philosophy for `jack-do` is: **Fast, Safe, and Respectful.**

- **Fast**: Use the best Rust parsers (OXC).
- **Safe**: Leverage Rust's borrow checker.
- **Respectful**: Don't break the user's formatting or delete their comments.
