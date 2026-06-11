# Phases: mcp-sequential-thinking

## Summary

The implementation is organized into 7 sequential phases. Each phase builds on the previous one and is independently testable. The work is linear (single agent) because each phase depends on types and traits defined in the prior phase.

## Dependency Graph

```text
Phase 1: Foundation
  └─→ Phase 2: Store
       └─→ Phase 3: Validation & Redaction
            └─→ Phase 4: Tools
                 ├─→ Phase 5: Export
                 └─→ Phase 6: Server Integration
                      └─→ Phase 7: Tests & Documentation
```

Phase 5 (Export) and Phase 4 (Tools) can be developed partially in parallel once the data model is stable. Phase 7 runs last to cover the complete system.

## Phase Index

| ID | Title | Depends On | Risk | Status |
|---|---|---|---|---|
| phase-1 | Foundation | — | Medium | Draft |
| phase-2 | In-Memory Store with Retention | phase-1 | Medium | Draft |
| phase-3 | Validation & Redaction | phase-2 | High | Draft |
| phase-4 | Tool Handlers | phase-3 | High | Draft |
| phase-5 | Export Formatters | phase-1 | Low | Draft |
| phase-6 | MCP Server Integration | phase-4, phase-5 | Medium | Draft |
| phase-7 | Test Suite & Documentation | phase-6 | Medium | Draft |

## Parallelization Notes

- Phase 5 (Export) only depends on Phase 1 (data models). It can be developed in parallel with Phases 2–4 if two agents are available.
- Phase 7 tests for Phases 1–5 can be written incrementally as each phase completes. Integration tests require Phase 6.
- For a single agent: execute phases sequentially. The total is ~8–12 hours of focused implementation.

## Shared File Risks

| File | Phases | Resolution |
|---|---|---|
| `src/model/mod.rs` | Phase 1, Phase 2, Phase 4, Phase 5 | Phase 1 defines it; later phases only add `pub mod` lines |
| `Cargo.toml` | Phase 1, Phase 6, Phase 7 | Phase 1 establishes dependencies; Phase 6 may add rmcp features; Phase 7 adds dev-dependencies |
| `src/main.rs` | Phase 6 | Only Phase 6 writes main.rs |

## Per-Phase Detail Files

- [phase-1](phases/phase-1.md) — Foundation: project skeleton, config, errors, data models
- [phase-2](phases/phase-2.md) — In-Memory Store with Retention
- [phase-3](phases/phase-3.md) — Validation & Redaction Pipeline
- [phase-4](phases/phase-4.md) — Tool Handlers
- [phase-5](phases/phase-5.md) — Export Formatters
- [phase-6](phases/phase-6.md) — MCP Server Integration
- [phase-7](phases/phase-7.md) — Test Suite & Documentation
