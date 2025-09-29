# Fluxum GitHub Copilot Instructions

## One-Liner Rules (Quick Reference)
- `let` = immutable, `mut` = mutable
- `fn` + `ret` for functions; explicit returns only
- Loops: `loop` + `skip` / `stop`; no `for` or `while`
- Parallel pipelines **default**: `map | filter | reduce | flatten | chunk | sliding_window`
- Sequential side-effects via **queue** (e.g., prints, debugging)
- Strong static typing; explicit `Option`/`Result` handling
- Minimal boilerplate; single-file scripts; no scaffolding
- Suggest **structure & patterns only**, **never exact code**
- Rust-inspired syntax; safe, clear, parallel-first

---

## General Guidelines
- Default variables are **immutable** (`let`). Use `mut` for mutable variables.
- Functions use `fn` with **explicit return (`ret`)**.
- Parallel pipelines are **default**.
- Sequential side-effects must use a **queue**.
- Keep code **minimal and idiomatic Rust**; avoid unnecessary boilerplate.
- Single-file scripts for v0.1; no scaffolding.
- Emphasize explicit Option/Result handling; no hidden exceptions.
- Do not provide exact code from AI or other sources; suggest **patterns and structure only**.

---

## Lexing / Parsing
- Tokenize: keywords, symbols, operators, literals, identifiers, comments.
- Track **line/column** for errors.
- Handle multi-character operators (`|>`, `==`, `!=`, `<=`, `>=`).
- Output: **stream of tokens** for parser.

---

## AST & Runtime
- AST nodes represent: functions, variables, loops, pipelines, match statements.
- Strong static typing only; explicit Option/Result handling.
- Runtime: lazy, parallel pipelines; sequential side-effects via queue.
- Standard library: `Files`, `Time`, `Regex`, `JSON`, `CSV`, `Env`.

---

## Iterators & Pipelines
- Default parallel-first using Rayon.
- Include: `map`, `filter`, `reduce`, `flatten`, `chunk`, `sliding_window`.
- Side-effects inside pipelines go through queue (prints, debugging).
- Avoid `for_each`; pipelines must remain chainable.

---

## CLI & Scripts
- Run scripts: `cargo run -- script.flx`.
- Optional shebang support: `#!/usr/bin/env fluxum`.
- Argument parsing optional (`clap`).

---

## Style & Philosophy
- Rust-inspired syntax; explicit, safe, minimal.
- No operator overloading or exceptions.
- Suggest structural guidance, not exact solutions.
- Encourage **experimentation and learning** with small modules.
- Prioritize clarity and type safety over clever shortcuts.
