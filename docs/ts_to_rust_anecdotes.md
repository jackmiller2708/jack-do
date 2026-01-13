Welcome! As a TypeScript developer, you might find Rust's strictness a bit daunting initially. This guide uses anecdotes from the `jack-do` codebase to help Bridge the gap.

For a deeper technical dive into these concepts, see our [Technical Rust Concepts Guide](rust_concepts.md).

## 1. Ownership: The Library Book vs. The Photocopy

**TS Developer Mindset**: In TypeScript, when you pass an object to a function, you're usually passing a "magic reference." Everyone has a key to the same room. If someone changes the furniture, everyone sees it.

**Rust Developer Mindset**: In Rust, data has one clear owner. Think of it like a **Library Book**. 
- You can **Move** the book (give it to someone else; you no longer have it).
- You can **Borrow** the book (let someone read it while you stand there; they can't keep it).
- You can **Photocopy** the book (`.clone()`), but then you have two separate books!

### Example from `typescript.rs`:
```rust
// The analyzer BORROWS the semantic model using '&'
let analyzer = UnusedDeclarationAnalyzer::new(&semantic);
```
In TS, you'd just pass `semantic`. In Rust, the `&` is a "Borrow," telling the compiler: "I'm just looking at this, I won't ruin it, and I'll give it back when I'm done."

### The "One-Way Ticket" (Moving)
Notice that `spans: Vec<Span>` doesn't have a `&`. This is a **Move**.

**TS Developer Mindset**: You don't usually think about "destroying" a variable. You just stop using it and wait for the GC.

**Rust Developer Mindset**: Think of it as giving someone a **One-Way Ticket**.
- When `process_file` calls `apply_modifications_to_file(..., spans)`, it hands over the entire list.
- `process_file` can **no longer use** that list of spans. It's gone!
- This is great because `apply_modifications_to_file` can now do whatever it wants with that data, and Rust will delete the list from memory the moment that function finishes. No GC needed!

### Destructuring: The Ultimate Hand-off
In TypeScript, destructuring `const { a, b } = obj` just gives you copies or references. The `obj` is still there, happy as a clam.

In Rust, destructuring a struct you **own** is like dismantling a LEGO set to build something else:
```rust
let ParserReturn { program, errors, .. } = Parser::new(...).parse();
```
- By doing this, you've **moved** `program` and `errors` into your local scope.
- The original "box" (`ParserReturn`) is gone. You've taken the pieces you wanted and tossed the box.
- You can do this because you **own** the return value of `parse()`. If you only had a borrow (`&ParserReturn`), Rust wouldn't let you "dismantle" it!

### The "Rules of the Road"
- **Move only if you Own**: You can't give away a book you've borrowed from the library. You can only give it away if you bought it (created it or it was given to you).
- **Borrow if you need it later**: If you have one slice of pizza and you "Move" it to a friend, you have zero slices. If you want a bite later, you have to let them "Borrow" it instead.

### The "Kitchen" Guide: When to do what?

| Action | Situation | Analogy | In TypeScript... |
| :--- | :--- | :--- | :--- |
| **Borrow** (`&`) | Read-only access | **Passing a recipe**: Everyone can read the recipe card, but no one writes on it or takes it home. | Most common. Passing an object to a function to read a property. |
| **Move** (Value) | Handing off task | **Giving away the ingredients**: You give the flour and eggs to the baker. You no longer have them; the baker now owns them. | Rare. Closest to deleting a variable after use or handing it to a worker thread. |
| **Mut Borrow** (`&mut`) | Modifying data | **Handing over the chef's knife**: Only one person can hold the knife at a time. They can sharpen it or use it, then they must give it back. | Any time you mutate an object property. But Rust ensures only ONE person does it at a time. |

#### "Why can't I just always borrow?"
If everything was a borrow, you'd have to manage the lifetime of *every* variable manually. Moving data "closer" to where it's used simplifies the logic. If a function owns its data, it doesn't have to ask the caller: "Are you still using this?" It just clears it when it's done.

---

## 2. Enums: The "Safe Parcel" vs. The "Magic Box"

**TS Developer Mindset**: You often use `null` or `undefined` to represent "nothing." You might use `try-catch` for errors. It's like a **Magic Box** that might contain a gift, or be empty, or explode when opened!

**Rust Developer Mindset**: Rust uses `Option` and `Result`. It's like a **Safe Parcel** with a label.
- A parcel labeled `Option` says: "I either have `Some(Gift)` or I'm `None`."
- A parcel labeled `Result` says: "I either have `Ok(Gift)` or an `Err(Explosion)`."

The "Catch": You **cannot** touch the gift inside until you've checked the label and safely opened it using `match` or `if let`.

### Example from `typescript.rs`:
```rust
// Instead of checking if (something === null)
if let Some(symbol_id) = ident.symbol_id.get() { ... }
```
This ensures we *never* have a "Null Pointer Exception." The compiler forces us to handle the `None` case.

---

## 3. Pattern Matching: The Superpowered Switch

**TS Developer Mindset**: You use `switch` or `if/else`, and maybe some basic destructuring.

**Rust Developer Mindset**: Rust's `match` is like a **Superpowered Switch**. It doesn't just check values; it can "look inside" those Safe Parcels we talked about.

### Example from `typescript.rs`:
```rust
match decl_node.kind() {
    AstKind::VariableDeclarator(_) => { /* Logic for vars */ }
    AstKind::ImportSpecifier(_) => { /* Logic for imports */ }
    _ => { /* Default case */ }
}
```
If you forget a case in TS, your app might crash at runtime. If you forget a case in Rust, the program **won't even compile**.

---

## 4. Lifetimes: The ID Badge with an Expiration Date

**TS Developer Mindset**: You rarely worry about how long a variable lives. The Garbage Collector (GC) cleans up after you.

**Rust Developer Mindset**: Rust doesn't have a GC. Instead, every reference has an **ID Badge with an Expiration Date** (a Lifetime).

### Example: `UnusedDeclarationAnalyzer<'a>`
The `'a` is that expiration date. It tells Rust: "This analyzer is holding onto a reference to the source code. It **must** be destroyed before the source code itself is deleted." This prevents your program from trying to read memory that has already been cleared.

---

## Summary Table

| Concept | TypeScript Mindset | Rust Mindset | Why it's cool |
| :--- | :--- | :--- | :--- |
| **Variables** | Valid by default | Owned by default | No double-free memory bugs |
| **References** | "Magic" pointers | Immutable borrows (`&`) | Thread-safety by default |
| **Errors** | `throw / try-catch` | `Result<T, E>` / `?` | Explicit, unskippable errors |
| **Empty Values** | `null / undefined` | `Option<T>` | No "Cannot read property of undefined" |

Moving to Rust isn't about learning a harder syntax; it's about learning a more **disciplined** way of thinking about your data. In TypeScript, the computer tries to guess what you mean. In Rust, you tell the computer exactly what you want, and it guarantees you'll get it safely.
