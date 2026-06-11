# Phase 5: Export Formatters

**Status:** Draft
**Depends on:** phase-1
**Risk:** Low
**Value:** Provides JSON, JSONL, and Markdown export of session traces. Can be developed in parallel with Phases 2–4.

## Goal

Implement formatters that convert a `ThinkingSession` into JSON, JSONL (one thought per line), and Markdown (human-readable) output.

## Produces

- `src/export/mod.rs`
- `src/export/json.rs`
- `src/export/jsonl.rs`
- `src/export/markdown.rs`

## Verification

```bash
cargo test --lib export        # Export formatter tests pass
```
