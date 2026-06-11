# mcp-sequential-thinking — Brainstorm Artifacts

**Contract Status:** Approved
**Confidence Score:** 100/100
**Created:** 2026-06-12

## Documents

- [Contract](contract.md) — What will be built, scope, constraints, decisions, risks
- [Phases](phases.md) — 7-phase implementation breakdown with dependency graph
- [Spec Index](spec/spec-index.md) — Implementation specs with reading order

## Contract Details

- [Problem](contract/problem.md)
- [Goals](contract/goals.md)
- [Success Criteria](contract/success-criteria.md)
- [Scope](contract/scope.md)
- [Constraints](contract/constraints.md)
- [Assumptions](contract/assumptions.md)
- [Decisions](contract/decisions.md)
- [Risks](contract/risks.md)

## Phase Details

- [Phase 1: Foundation](phases/phase-1.md)
- [Phase 2: In-Memory Store with Retention](phases/phase-2.md)
- [Phase 3: Validation & Redaction](phases/phase-3.md)
- [Phase 4: Tool Handlers](phases/phase-4.md)
- [Phase 5: Export Formatters](phases/phase-5.md)
- [Phase 6: MCP Server Integration](phases/phase-6.md)
- [Phase 7: Test Suite & Documentation](phases/phase-7.md)

## Implementation Specs

- [Spec Phase 1: Foundation](spec/spec-phase-1.md)
- [Spec Phase 2: Store](spec/spec-phase-2.md)
- [Spec Phase 3: Validation](spec/spec-phase-3.md)
- [Spec Phase 4: Tools](spec/spec-phase-4.md)
- [Spec Phase 5: Export](spec/spec-phase-5.md)
- [Spec Phase 6: Server](spec/spec-phase-6.md)
- [Spec Phase 7: Tests & Docs](spec/spec-phase-7.md)

## Agent Artifacts

- [Manifest](../../.agent/contracts/mcp-sequential-thinking/manifest.json)
- [Contract NDJSON](../../.agent/contracts/mcp-sequential-thinking/contract.ndjson)
- [Confidence NDJSON](../../.agent/contracts/mcp-sequential-thinking/confidence.ndjson)
- [Phases NDJSON](../../.agent/contracts/mcp-sequential-thinking/phases.ndjson)
- [Specs Index NDJSON](../../.agent/contracts/mcp-sequential-thinking/specs.index.ndjson)
- [Decisions NDJSON](../../.agent/contracts/mcp-sequential-thinking/decisions.ndjson)
- [Risks NDJSON](../../.agent/contracts/mcp-sequential-thinking/risks.ndjson)

## Next Step

Run `implement` skill to execute the 7 phases and build the MCP server.
