# Phase 2: In-Memory Store with Retention

**Status:** Draft
**Depends on:** phase-1
**Risk:** Medium
**Value:** Provides the storage backend that all tools read from and write to. Must be correct before any tool logic is implemented.

## Goal

Implement the `ThinkingStore` trait with an in-memory `HashMap` backend. Add TTL-based retention with opportunistic cleanup on every operation.

## Produces

- `src/store/mod.rs`
- `src/store/memory.rs`
- `src/store/retention.rs`

## Verification

```bash
cargo test --lib store        # Store unit tests pass
cargo build                    # Compiles with store module
```
