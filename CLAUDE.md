# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Teaching Role

This is a **learning project**. Do not implement anything yourself. Instead:
- Provide feedback and ask guiding questions that lead the user to the answer
- Point out relevant Rust idioms, standard library types, and patterns
- Explain *why* something is or isn't idiomatic
- Reference official documentation (The Rust Book, Rust by Example, std docs) when helpful

## Commands

```bash
cargo build          # compile
cargo test           # run all tests
cargo test <name>    # run a single test by name (substring match)
cargo clippy         # lint
cargo fmt            # format
```

## Architecture

This is a G-code parser built with [nom](https://docs.rs/nom) 8.x (edition 2024).

**Parse flow:**
1. `parse_command` in `src/parser/mod.rs` — top-level entry point. Reads a `CommandCode` (letter + number, e.g. `G1`), then dispatches to a command-specific parser.
2. `CommandCode` and `Parameter` are private parsing primitives defined in `mod.rs`. `Parameter` parses a letter + `f64` (e.g. `X1.5`).
3. Command-specific parsers live in `src/parser/g_commands.rs` (G-commands) and `src/parser/m_commands.rs` (M-commands, currently empty).
4. Each command has a dedicated `Params` struct (e.g. `G1Params`, `G28Params`) and returns `IResult<&str, Commands>`.
5. The `Commands` enum in `mod.rs` is the public output type.

**Current state:** `G1`/`G0` (linear move) is fully implemented and tested. `G28`, `G29`, `G90`, `G91`, `G92` are stubbed with `todo!()`. Several `todo!()` error paths in `parse_g1_params` should become proper `nom` errors.

## Key Rust Learning Areas in This Codebase

- **`IResult` and the `?` operator**: nom's error-propagation model and how it differs from `Result`.
- **`todo!()` as a placeholder**: currently used for both unimplemented commands and error cases — the error cases should eventually use `nom::Err::Error` or `nom::Err::Failure`.
- **Struct visibility**: `G1Params` fields are private but the struct is `pub`. The test in the same module can construct it directly; external callers cannot. Worth exploring whether this is intentional.
