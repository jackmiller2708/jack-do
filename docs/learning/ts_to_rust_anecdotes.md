# My TypeScript to Rust Journey

> [!IMPORTANT]
> **Personal Learning Notes**: This document contains my personal findings and "Aha!" moments while learning Rust. It is not an official Rust guide or standard learning material. For official documentation, please visit [doc.rust-lang.org](https://doc.rust-lang.org).

Building `jack-do` as my first real Rust project has been quite an adventure. Coming from a TypeScript background, I had to unlearn a few things and embrace new ways of thinking. These are the anecdotes and comparisons that helped me finally "get" Rust.

For a deeper technical dive into my notes, see my [Technical Rust Learning Guide](rust_concepts.md).

## 1. Ownership: The Library Book vs. The Photocopy

**How I used to think (TS)**: When I passed an object to a function, I just thought of it as a "reference." Everyone could touch it. It was like everyone having a key to the same room.

**The "Aha!" Moment (Rust)**: Every piece of data has one clear owner. I started thinking of it like a **Library Book**. 
- I can **Move** the book (give it to someone else; I no longer have it).
- I can **Borrow** the book (let someone read it while I wait; they have to give it back).
- I can **Photocopy** the book (`.clone()`), but that's a whole separate copy.

### Example from my code:
```rust
// I'm BORROWING the semantic model here using '&'
let analyzer = UnusedDeclarationAnalyzer::new(&semantic);
```
In TS, I'd just pass `semantic`. In Rust, I realized the `&` is me telling the compiler: "I'm just letting the analyzer look at this, but I'm keeping it for later."

### The "One-Way Ticket" (Moving)
I was really confused when I first saw `spans: Vec<Span>` without a `&`. At first, I didn't even notice I could do that! It wasn't until I tried a few different ways of passing the data—and the compiler corrected me—that it finally clicked. This is what's known as a **Move**.

**My discovery**: 
- I spent a lot of time "borrowing" everything by default. But then I hit cases where I wanted the function to just *take* the data and be done with it.
- When I call `apply_modifications_to_file(..., spans)`, I'm handing over the entire list.
- Because I'm not using `&`, I'm telling Rust: "Here, take this. I don't need it back."
- This was a huge win for my code's efficiency, and I only figured it out by listening to the compiler's suggestions when my initial "borrowing" approach got too complex.

### Destructuring: Dismantling the Box
In Rust, destructuring a struct that a scope **owns** is like dismantling a LEGO set to build something else:
```rust
let ParserReturn { program, errors, .. } = Parser::new(...).parse();
```
- By doing this, the `program` and `errors` variables have **moved** into the local scope.
- The original "box" (`ParserReturn`) is gone. The pieces needed were taken, and the box was tossed.
- This is possible because the `process_file` function owns the return value of `parse()`. If there were only a borrow (`&ParserReturn`), Rust wouldn't let the code "dismantle" it!

### My "Rules of the Road" for Myself:
- **Move only if it is Owned**: You can't give away a book you've borrowed.
- **Borrow if the scope needs it later**: if a variable needs to be used again, it must be borrowed with `&`.

### My "Kitchen" Interpretation: When to do what?

| Action | My Situation | Analogy |
| :--- | :--- | :--- |
| **Borrow** (`&`) | I just need to read it | **Passing a recipe**: Everyone reads it, but no one takes it home. |
| **Move** (Value) | I'm handing off a task | **Giving away the ingredients**: I give them to the baker; they are no longer mine. |
| **Mut Borrow** (`&mut`) | I need someone to fix it | **Handing over the chef's knife**: Only one person can hold it at a time. They fix it/use it, then give it back. |

---

## 2. Enums: The "Safe Parcel" vs. The "Magic Box"

**How I used to think (TS)**: I used `null` or `undefined` everywhere. It was like a **Magic Box** that might be empty, might have a value, or might crash my app!

**The "Aha!" Moment (Rust)**: Rust uses `Option` and `Result`. It's like a **Safe Parcel** with a clear label.
- Label `Option`: "I'm either `Some(Gift)` or I'm `None`."
- Label `Result`: "I'm either `Ok(Gift)` or an `Err(Explosion)`."

The best part? I'm **not allowed** to touch the gift until I've checked the label and safely opened it using `match` or `if let`.

---

## 3. Pattern Matching: My New Favorite Tool

**How I used to think (TS)**: I used lots of `if/else` or simple `switch` statements.

**The "Aha!" Moment (Rust)**: Rust's `match` is like a **Superpowered Switch**. It doesn't just check values; it lets me peer into those Safe Parcels.

### Example I'm proud of:
```rust
match decl_node.kind() {
    AstKind::VariableDeclarator(_) => { /* ... */ }
    AstKind::ImportSpecifier(_) => { /* ... */ }
    _ => { /* handles everything else */ }
}
```
In TS, if I forgot a case, I'd find out at runtime. In Rust, the compiler yells at me before I even run the code. I love that safety net.

---

## 4. Lifetimes: The ID Badge with an Expiration Date

**My biggest struggle**: Lifetimes were the hardest part for me to grasp.

**My Interpretation**: Every reference I use has an **ID Badge with an Expiration Date** (a Lifetime).

### Example: `UnusedDeclarationAnalyzer<'a>`
The `'a` is that expiration date. It tells Rust: "This analyzer is holding onto a reference to the source code. It **must** be gone before the source code itself is deleted." I realized this is how Rust prevents my program from trying to read memory that's already been cleaned up.

---

## 5. The Strict Compiler: My Best Friend (and Worst Enemy)

Coming from TS, the Rust compiler felt like a brick wall at first. But then I realized: **it's checking everything so I don't have to.**

- In TS, "If it compiles, it *might* run."
- In Rust, "If it compiles, it **will** run (safely)."

The compiler checks for memory leaks, null pointers, and data races before I even get an executable. It's like having a senior developer reviewing every single line of my code in real-time. Even when it yells at me about Borrowing or Lifetimes, it's usually pointing out a bug I would have spent days chasing in TypeScript.

## My Reflection
Moving to Rust isn't just about a different syntax; it's about learning a more **disciplined** way of thinking. In TypeScript, I felt like the computer was trying to guess what I meant. In Rust, I tell the computer *exactly* what I want, and it guarantees I'll get it safely. 

This project is a record of those lessons. I hope my notes help you too!
