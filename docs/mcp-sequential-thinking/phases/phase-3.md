# Phase 3: Validation & Redaction

**Status:** Draft
**Depends on:** phase-2
**Risk:** High
**Value:** The validation pipeline is the core correctness logic. Every thought passes through 17 validation steps. Redaction is a critical privacy requirement.

## Goal

Implement the full 17-step validation pipeline. Implement redaction patterns for 10+ secret types. Implement default value application per the spec's status and kind tables.

## Produces

- `src/validation/mod.rs`
- `src/validation/rules.rs`
- `src/validation/defaults.rs`
- `src/redaction/mod.rs`
- `src/redaction/patterns.rs`

## Verification

```bash
cargo test --lib validation    # Validation tests pass
cargo test --lib redaction     # Redaction tests pass (all 10+ patterns)
```
