# Phase 1: Foundation

**Status:** Draft
**Depends on:** None
**Risk:** Medium
**Value:** Establishes all types, traits, and configuration that every other phase depends on. Nothing else can start before this phase completes.

## Goal

Create the Rust project skeleton with pinned dependencies, configuration loading, error types, and all data model enums/structs with correct serde and schemars derives.

## Produces

- `Cargo.toml` with pinned dependencies
- `src/main.rs` (stub)
- `src/config.rs`
- `src/error.rs`
- `src/model/mod.rs`
- `src/model/session.rs`
- `src/model/thought.rs`
- `src/model/branch.rs`
- `src/model/evidence.rs`
- `src/model/risk.rs`
- `src/model/action.rs`
- `src/model/budget.rs`
- `src/model/redaction.rs`
- `src/model/warning.rs`
- `.gitignore`

## Verification

```bash
cargo build          # Compiles without errors
cargo check          # Type-check passes
```
