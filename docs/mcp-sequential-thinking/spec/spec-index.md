# Spec Index: mcp-sequential-thinking

## Summary

Seven sequential implementation specs. Each builds on the prior one. Phase 5 (Export) can run in parallel with Phases 2–4 if multiple agents are available.

## Specs

| Spec | Title | Depends On | Purpose | Status |
|---|---|---|---|---|
| spec-phase-1.md | Foundation | — | Project skeleton, config, errors, data model types | Draft |
| spec-phase-2.md | In-Memory Store with Retention | Phase 1 | ThinkingStore trait, HashMap backend, TTL cleanup | Draft |
| spec-phase-3.md | Validation & Redaction | Phase 2 | 17-step validation pipeline, 10+ redaction patterns | Draft |
| spec-phase-4.md | Tool Handlers | Phase 3 | Handler logic for all 6 MCP tools | Draft |
| spec-phase-5.md | Export Formatters | Phase 1 | JSON, JSONL, Markdown export | Draft |
| spec-phase-6.md | MCP Server Integration | Phase 4, Phase 5 | rmcp wiring, tool registration, main.rs | Draft |
| spec-phase-7.md | Test Suite & Documentation | Phase 6 | 10 test categories, README, LICENSE | Draft |

## Recommended Reading Order

1. `../contract.md` — what and why
2. `../phases.md` — phase breakdown and dependencies
3. `spec-phase-1.md` through `spec-phase-7.md` in order
4. Reference: `.agent/contracts/mcp-sequential-thinking/specs.index.ndjson` for agent loading

## Agent Loading Guidance

Implementation agents should start with:

1. `.agent/contracts/mcp-sequential-thinking/manifest.json`
2. `.agent/contracts/mcp-sequential-thinking/specs.index.ndjson`
3. The specific phase spec assigned to them
4. `.agent/contracts/mcp-sequential-thinking/contract.ndjson` for constraints and decisions
