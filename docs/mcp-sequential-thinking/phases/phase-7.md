# Phase 7: Test Suite & Documentation

**Status:** Draft
**Depends on:** phase-6
**Risk:** Medium
**Value:** Validates the complete system against the spec and provides user-facing documentation.

## Goal

Write the full test suite (10 categories) and project documentation (README, LICENSE). All tests must pass. Cargo.toml must have pinned versions.

## Produces

- `tests/compatibility.rs`
- `tests/sessions.rs`
- `tests/branches.rs`
- `tests/revisions.rs`
- `tests/dependencies.rs`
- `tests/budgets.rs`
- `tests/redaction.rs`
- `tests/retention.rs`
- `tests/export.rs`
- `tests/mcp_contract.rs`
- `README.md`
- `LICENSE`

## Verification

```bash
cargo test                     # All tests pass
cargo clippy -- -D warnings   # No clippy warnings
cargo fmt -- --check           # Proper formatting
```
