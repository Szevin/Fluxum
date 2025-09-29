# Fluxum

**Parallel-first, Rust-inspired scripting language with strong types and lazy pipelines**

Fluxum is a minimal scripting language designed for safe, parallel-first computation.
It combines the strong typing and pattern matching of Rust with the high-level expressiveness and functional programming capabilities of Python.
Designed for experimentation and rapid iteration, Fluxum encourages explicit handling of Options/Results and parallel computation by default.

---

## Features

- Immutable variables by default (`let`) with optional mutability (`mut`)
- Functions declared with `fn` and explicit return using `ret`
- Loops with `loop`, controlled using `skip` and `stop`
- Pattern matching via `match` for Option/Result types
- Parallel-first lazy iterators: `map`, `filter`, `reduce`, `flatten`, `chunk`, `sliding_window`
- Sequential side-effects queue for debugging (`dbg_seq`)
- Minimal standard library: `Files`, `Time`, `Regex`, `JSON`, `CSV`, `Env`
- CLI runner for executing scripts from the terminal
- Explicit handling of errors; no hidden exceptions

---

## Installation

**Requirements:**

- Rust 1.XX+
- Cargo

**Build & Run:**

```bash
git clone https://github.com/yourusername/fluxum.git
cd fluxum
cargo build --release
cargo run -- path/to/script.flx
