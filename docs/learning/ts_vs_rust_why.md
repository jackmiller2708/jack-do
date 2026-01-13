# Why Rust?

> [!IMPORTANT]
> **Personal Learning Notes**: This document contains my personal findings and "Aha!" moments while learning Rust. It is not an official Rust guide or standard learning material. For official documentation, please visit [doc.rust-lang.org](https://doc.rust-lang.org).

If you're on the fence about Rust, you might be thinking: *"TypeScript is fast enough, and the DX is great. Why bother with the steep learning curve?"*

I had the same thoughts. But as I built `jack-do`, I discovered that Rust isn't just about raw speed—it's about a level of **software discipline** and **architectural clarity** that TypeScript simply can't provide.

## 1. Safety: Ending the "Guessing Game"

**TypeScript**: Even with high strictness settings, TS is a "best-effort" wrapper around JavaScript. You often find yourself wondering: *"Can this be null here?"* or *"Did I remember to handle that edge case?"*. It's a game of guessing where the holes in your logic might be.

**Rust**: The compiler is a high-stakes gatekeeper. It doesn't guess; it **proves**.
- **Exhaustive Matching**: If you add a new field to an Enum, every single part of the codebase that uses it will fail to compile until you handle the change. No more "runtime surprises."
- **Null-Safety**: Rust doesn't have `null` or `undefined`. You are *forced* to handle the absence of data explicitly, turning potential runtime crashes into simple compile-time chores.

## 2. DX: Explicitness over Ambiguity

**TypeScript**: TypeScript often tries to be "helpful" by inferring types or allowing flexible patterns. While this feels fast initially, it leads to ambiguity. You often have to "Peel the onion" (click through 5 type definitions) to understand what a variable actually contains.

**Rust**: **Explicit is better than implicit.**
- You always know exactly who owns a piece of data and how long it lives.
- While it requires more typing (being explicit about borrows and results), it eliminates the mental load of "reasoning" about the code. 
- **The Result**: You spend more time thinking *before* you code, and almost zero time debugging *after* you code. The logic becomes self-documenting.

## 3. Immutability: Built-in vs. The "Overhead Tax"

| Feature | TypeScript (ImmutableJS/immer) | Rust (Native) |
| :--- | :--- | :--- |
| **Default State** | Mutability is the default. | **Immutability is the default**. |
| **Immutability Cost** | **High**. Libraries like ImmutableJS create entire copies of objects or use complex proxy systems, adding significant CPU and memory overhead. | **Zero**. Rust's borrow checker enforces immutability at compile time with *zero* runtime overhead. |

**My Discovery**: In `jack-do`, I don't need a library to make my AST processing safe. I can pass a reference as `&`, and the compiler **guarantees** nobody can change it. If I need to change it, I must explicitly mark it as `&mut`. You get perfectly safe, predictable data flow without the "Immutability Tax" that slows down Big TS apps.

## 4. Performance Discovery (The Hardware Advantage)

### Garbage Collection vs. Ownership
- **TS**: Uses a Garbage Collector. This can cause random "latency spikes" as the collector stops the world to clean up.
- **Rust**: Uses Ownership. Memory is cleaned up the microsecond it's no longer needed. The performance is "flat" and predictable.

### Objects vs. Packed Structs
- **TS**: Objects are heavy dictionaries in memory.
- **Rust**: Structs are packed tightly. This allows my CLI to fit thousands of AST nodes into the CPU's core cache, making analysis feel "instant" while TS would be waiting for slow RAM.

## 5. Startup and Concurrency

- **Startup**: Rust binaires start in ~1ms. Node.js takes ~100-300ms just to wake up. For a CLI, this is the difference between "feeling native" and "feeling laggy."
- **Fearless Concurrency**: In TS, multi-threading (Worker Threads) is heavy and scary because you might have race conditions. In Rust, if your multi-threaded code compiles, the compiler has **mathematically proven** there are no data races.

## The "Limits" of TypeScript

TypeScript is amazing for the web, but it’s a language of **convenience**. Rust is a language of **discipline**.

Rust forces you to be a better engineer by being explicit about memory, mutation, and errors. In return, it gives you a level of performance and reliability that TypeScript's foundations—where I was often playing a "Guessing Game" about execution and state—can never reach. If you want to build tools that are not just "fast enough" but truly **optimal**, Rust is the upgrade you've been looking for.

### As an Engineer started out with JS
If I could relearn programming all over again, I’d skip the "easy" start and go straight to Rust. It would have shaped a more robust mindset from day one and made my thought process far more explicit. It's the best way to truly understand what a computer is doing with your code. Now that I’ve experienced Rust’s discipline, I’d never go back to plain JavaScript. If I ever use TypeScript, I’ll only build it on top of the **Effect library**, as it’s the only thing that brings that same level of safety and joy I found in Rust.
