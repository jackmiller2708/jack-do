# Rust Concepts in Jack-Do 🦀

This guide explains the Rust concepts used in the `jack-do` codebase. Rust's features ensure that our CLI is fast, memory-safe, and reliable.

## 1. Ownership and Borrowing
Ownership is Rust's most unique feature. It manages memory without a garbage collector.

### How it works in `process_file`:
```rust
async fn process_file(path: &Path) -> Result<()> {
    let source_text = fs::read_to_string(path)?; // source_text "owns" the string data
    let allocator = Allocator::default();
    
    // We pass &source_text (a reference) to the Parser. 
    // The Parser "borrows" the text but doesn't take ownership.
    let ParserReturn { program, .. } = Parser::new(&allocator, &source_text, ...).parse();
}
```
> [!NOTE]
> By borrowing instead of moving, we can keep using `source_text` later in the function to apply modifications.

### 1.1 Moving: The One-Way Ticket
Sometimes we *want* to give away ownership. This is a "Move." In `apply_modifications_to_file`, we pass `spans: Vec<Span>` without a `&`.

```rust
fn apply_modifications_to_file(path: &Path, source: &str, spans: Vec<Span>)
```

**Why no `&`?**
1. **Efficiency**: A `Vec` is just a pointer, a capacity, and a length. Moving it is just copying those 24 bytes.
2. **Usage**: We don't need the `spans` list anymore after this function finishes. By moving it, the function "owns" the list and will automatically clean up the memory as soon as it's done, without the caller having to worry about it.

### 1.2 The Golden Rules: When can we Move?

In Rust, you can only **Move** data if you **Own** it.

- **You can Move if**: You created the variable (e.g., `let x = String::new()`) or if it was moved to you by a function you called (e.g., `let x = fs::read_to_string(...)`).
- **You MUST Borrow if**: You don't own the data (it was passed to you as `&T`) or if you need to use the data again later in the same function.

### 1.3 Destructuring as a Move

A common place where Moving happens is during **Destructuring**. In `process_file`, we destructure the result of `parse()`:

```rust
let ParserReturn { program, errors, .. } = Parser::new(...).parse();
```

**What's happening here?**
1. `parse()` returns a `ParserReturn` struct (it moves ownership to us).
2. We immediately destructure it. 
3. **Important**: Because we didn't use `&ParserReturn`, the fields `program` and `errors` are **moved** out of the struct and into our local variables.
4. The rest of the struct (the `..` part) is immediately dropped/deleted because it no longer has an owner.

This is a powerful way to "dismantle" an object and take only what you need, efficiently cleaning up the rest.

### 1.4 Decision Matrix: Move vs. Borrow

Deciding whether to take a reference (`&T`) or the value itself (`T`) is a core part of Rust API design.

#### Use **Borrowing** (`&T`) when:
- **Shared Access**: Multiple parts of the code need to read the same data (e.g., `source_text` is borrowed by both the analyzer and the modification logic).
- **Read-Only**: You only need to inspect the data without changing it.
- **Large Data**: Copying a 10MB string is slow; borrowing a reference to it is nearly instant.
- **Staying in Scope**: You want the data to stay with the caller so they can use it again.

#### Use **Moving** (`T`) when:
- **Ownership Transfer**: You are "handing off" a task to another function (e.g., sending data to a background thread or a final cleanup step).
- **Transformation**: You want to consume the old data and turn it into something new (e.g., `vec.into_iter()`).
- **Small "Copyable" Types**: Primitives like `i32`, `bool`, and `char` have the `Copy` trait. They are so small that Rust just copies them automatically instead of moving them. For these, "Move" and "Copy" look identical.
- **Final Destination**: Like `apply_modifications_to_file`, once the data reaches its final step, moving it allows the function to be self-contained and manage its own cleanup.

> [!TIP]
> **What about Mutation?**
> If you need to *change* data, you use a **Mutable Borrow** (`&mut T`).
> - Rule: You can have **many readers** OR **exactly one writer**, but never both at once. 
> - This simple rule is why Rust is immune to "Data Races" in multi-threaded code.

## 2. Lifetimes (`'a`)
Lifetimes ensure that references remain valid as long as they are needed.

### In `UnusedDeclarationAnalyzer<'a>`:
```rust
struct UnusedDeclarationAnalyzer<'a> {
    semantic: &'a oxc_semantic::Semantic<'a>,
}
```
The `'a` tells the Rust compiler: "This analyzer cannot live longer than the `Semantic` model it references." This prevents "dangling pointers"—a common source of bugs in other languages.

## 3. Error Handling (`Result` and `?`)
Instead of exceptions, Rust uses the `Result` type.

- `Result<T, E>` is an enum: `Ok(value)` or `Err(error)`.
- The `?` operator is shorthand: "If this failed, return the error immediately; otherwise, give me the value."

### Example:
```rust
let source_text = fs::read_to_string(path)?; // Returns early if file read fails
```

## 4. Pattern Matching
The `match` statement is like a powerful `switch` on steroids. It's used extensively to navigate the TypeScript Abstract Syntax Tree (AST).

### Example from `find_unused_spans`:
```rust
match decl_node.kind() {
    AstKind::VariableDeclarator(_) => { /* logic */ }
    AstKind::BindingIdentifier(_) => { /* logic */ }
    _ => { /* default case */ }
}
```
Rust enforces "exhaustiveness," meaning you must handle every possible case (or use `_`), ensuring no edge cases are missed.

## 5. Structs and Impl Blocks
Rust separates data (structs) from logic (impl blocks). This promotes a clean, object-oriented-like structure while remaining data-centric.

- **Struct**: Defines the "shape" of the data.
- **Impl**: Defines "behavior" (methods) for that data.

## 6. Functional Programming with Iterators
Rust's iterators are "lazy" and highly optimized.

```rust
var_decl.declarations.iter().all(|d| self.is_pattern_entirely_unused(&d.id))
```
This expressive code is often as fast as a manual `for` loop, thanks to "zero-cost abstractions."

---

### Why Rust for programs like `jack-do`?
1. **Performance**: No runtime or garbage collector means it runs at C/C++ speeds.
2. **Reliability**: The compiler catches most bugs (null pointers, race conditions) before the code even runs.
3. **Tooling**: `cargo` handles building, testing, and dependency management seamlessly.
