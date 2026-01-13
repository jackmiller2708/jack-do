# My Rust Learning Notes: Concepts in Jack-Do

These are the notes I've taken while building `jack-do`. Coming from TypeScript, some of these concepts felt like obstacles at first, but now I see them as the features that make my CLI fast and safe.

## 1. Ownership and Borrowing: The Biggest "Aha!" Moment
Ownership was the first big hurdle I encountered. It's how Rust manages memory without a garbage collector—it was quite a shift from how I used to think in JS.

### How it works in `process_file`:
```rust
async fn process_file(path: &Path) -> Result<()> {
    let source_text = fs::read_to_string(path)?; // The `source_text` variable owns the string data
    let allocator = Allocator::default();
    
    // The &source_text reference is passed to the Parser. 
    // The Parser "borrows" the text but doesn't take ownership.
    let ParserReturn { program, .. } = Parser::new(&allocator, &source_text, ...).parse();
}
```
> [!NOTE]
> I realized that by borrowing instead of moving, the `source_text` variable keeps its ownership so I can use it later in the function to apply modifications. If the parser had taken ownership, the string would have been "consumed" and dropped when the parser finished!

### 1.1 Moving: The One-Way Ticket
I eventually learned that sometimes I *want* to give away ownership. I call this the "One-Way Ticket." In `apply_modifications_to_file`, I pass `spans: Vec<Span>` without a `&`.

```rust
fn apply_modifications_to_file(path: &Path, source: &str, spans: Vec<Span>)
```

**What I discovered about why I don't need a `&` here:**
1. **The Compiler's Suggestion**: I only realized I could move this because the compiler stopped me when I tried to borrow it in a way that made things too complicated. It suggested that since the `process_file` scope owns the `spans` variable, it can just "give it away" to the next function.
2. **Efficiency**: A `Vec` is just a pointer, a capacity, and a length. Moving it is just copying those 24 bytes.
3. **Cleanup**: The `process_file` function doesn't need the `spans` list anymore after this call. By moving it, the `apply_modifications_to_file` function becomes the new owner and handles the cleanup.

### 1.2 The Golden Rules I'm following:
Through trial and error, I found that you can only **Move** data if you **Own** it.

- **I can Move if**: I created the variable or if it was moved to me by a function I called.
- **I MUST Borrow if**: I don't own the data or if I need to use it again later in the same function.

### 1.3 Destructuring: A Cool Way to Move
I was surprised to find that **Destructuring** can be a Move. In `process_file`, when the code destructures the result of `parse()`:

```rust
let ParserReturn { program, errors, .. } = Parser::new(...).parse();
```

**How I understand this now:**
1. `parse()` hands back a `ParserReturn` struct (the `process_file` scope now owns it).
2. By destructuring it without `&`, the `program` and `errors` fields are **moved** out of the struct and into new local variables.
3. The rest of the struct (the `..` part) is tossed away. It's like taking the toys out of the box and immediately recycling the cardboard!

### 1.4 My Decision Matrix: Move vs. Borrow

This is the mental checklist I use when deciding between a reference (`&T`) and the value itself (`T`).

#### I use **Borrowing** (`&T`) when:
- **Shared Access**: Multiple parts of my code need to read the same data.
- **Read-Only**: I only need to inspect the data without changing it.
- **Large Data**: I want to avoid the cost of copying large strings or buffers.
- **Staying in Scope**: I want to keep the data so I can use it again.

#### I use **Moving** (`T`) when:
- **Ownership Transfer**: I'm done with the data and want to hand it off.
- **Transformation**: I want to consume the data and turn it into something else.
- **Small "Copyable" Types**: Primitives like `i32` or `bool`. They are so small that Rust copies them automatically—no "Move" needed.
- **Final Destination**: Like my `apply_modifications_to_file` function, once the data reaches its final step, moving it makes the cleanup automatic.

> [!TIP]
> **Mutation Discovery**
> When I need to *change* data, I use a **Mutable Borrow** (`&mut T`).
> The "Exactly one writer OR many readers" rule was a bit of a shock, but it's why I don't have to worry about weird state bugs or data races.

## 2. Lifetimes (`'a`): Decoding the Mystery
Lifetimes were intimidating at first. I've learned they just ensure that references remain valid as long as they are needed.

### My use in `UnusedDeclarationAnalyzer<'a>`:
```rust
struct UnusedDeclarationAnalyzer<'a> {
    semantic: &'a oxc_semantic::Semantic<'a>,
}
```
I now understand that the `'a` tells the compiler: "This analyzer cannot outlive the `Semantic` model it's looking at." It's like a safety tether.

## 3. Error Handling: No More Try-Catch!
Instead of exceptions, I'm using the `Result` type. It's much more explicit.

- `Result<T, E>` is an enum: `Ok(value)` or `Err(error)`.
- The `?` operator is my favorite feature—it's like shorthand for "bubble this error up if it happens."

### Example from my code:
```rust
let source_text = fs::read_to_string(path)?; // Clean and easy!
```

## 4. Pattern Matching: Navigating the AST
The `match` statement is like a super-powered `switch`. I've used it everywhere to navigate the TypeScript AST.

### My logic in `find_unused_spans`:
```rust
match decl_node.kind() {
    AstKind::VariableDeclarator(_) => { /* ... */ }
    AstKind::BindingIdentifier(_) => { /* ... */ }
    _ => { /* default */ }
}
```
The "Exhaustiveness" requirement is great—it means I can't accidentally forget to handle a case.

## 5. Structs and Impl: Separating Data and Logic
I've learned to separate the "shape" of my data (structs) from the "behavior" (impl blocks). It's very clean and makes it easier to organize my thoughts.

- **Struct**: Where I define the data.
- **Impl**: Where I define what that data can *do*.

## 6. Functional Style: The Beauty of Iterators
Coming from TS, I found Rust's iterators very familiar but even more powerful.

```rust
var_decl.declarations.iter().all(|d| self.is_pattern_entirely_unused(&d.id))
```
I love that this syntax is so expressive yet runs at the speed of a manual `for` loop.

---

### Why I'm sticking with Rust for this journey:
1. **Performance**: I'm amazed at how fast it is without a GC.
2. **The Strict Compiler is a Superpower**: It checks every possible edge case before letting me compile. It's strict, but it prevents 90% of the bugs I used to spend hours debugging in other languages. I've learned to trust it as a guide rather than seeing it as a hurdle.
3. **Cargo**: It just works. Dependency management is a dream compared to what I was used to.
