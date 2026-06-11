# Phase 4: Tool Handlers

**Status:** Draft
**Depends on:** phase-3
**Risk:** High
**Value:** The tool handlers are the business logic for all 6 MCP tools. This is the largest and most complex phase.

## Goal

Implement handler functions for all 6 MCP tools: `sequentialthinking`, `sequential_thinking` (alias), `get_thinking_session`, `list_thinking_sessions`, `clear_thinking_session`, `export_thinking_session`.

## Produces

- `src/tools/mod.rs`
- `src/tools/sequentialthinking.rs`
- `src/tools/get_session.rs`
- `src/tools/list_sessions.rs`
- `src/tools/clear_session.rs`
- `src/tools/export_session.rs`

## Verification

```bash
cargo test --lib tools         # Tool handler unit tests pass
```
