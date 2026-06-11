# Contract: mcp-sequential-thinking

**Status:** Approved
**Created:** 2026-06-12
**Updated:** 2026-06-12
**Confidence Score:** 100/100
**Project Slug:** mcp-sequential-thinking
**Source Input:** Detailed product specification for a single-shot complete Rust MCP server
**Target Repository:** /Volumes/Workspace/rnd/workflow/mcp/sequential-thinking

## Summary

Build a complete Rust MCP server over stdio that provides structured reasoning traces for AI agents. The server records typed thinking steps (observations, assumptions, decisions, revisions, etc.) with branching, dependency tracking, evidence references, risk classification, and action proposals. It enforces budgets, TTL retention, and secret redaction by default. Delivered as a single-shot complete implementation — no MVP or staged versions.

## Confidence Summary

| Dimension | Score | Reason |
|---|---:|---|
| Problem Clarity | 20 | Free-form thought logs are explicitly contrasted with the desired structured reasoning trace; affected users (AI agents), current workaround, and impact are all defined |
| Goal Definition | 20 | 34 required features enumerated; 30+ binary completion criteria; every data model, tool schema, and validation rule specified with examples |
| Success Criteria | 20 | Section 36 defines explicit completion criteria per component; Section 35 specifies test categories and expected behaviors; validation pipeline order is explicit |
| Scope Boundaries | 20 | 11 explicit non-goals (Section 4); 34 required features (Section 5); security boundaries in Section 34; no ambiguity |
| Consistency | 20 | Kind defaults align with status table; revision model matches branch patterns; budget output matches config; error codes match validation rules; JSON examples match Rust types |

## Problem Statement

Existing Sequential Thinking MCP servers only record free-form text thoughts. AI agents performing complex multi-step tasks need structured reasoning: typed thought kinds, branching for alternatives, revision tracking, dependency chains, evidence references, risk classification, and budget enforcement. Without structure, reasoning traces are not auditable, revisable, or machine-actionable.

Detailed version: `contract/problem.md`

## Goals

1. Build a Rust stdio MCP server exposing the `sequentialthinking` tool with typed thought kinds
2. Support structured reasoning with branches, revisions, dependencies, evidence, risk, and action proposals
3. Enforce configurable budgets, TTL retention, and secret redaction
4. Support legacy compatibility with the original `sequential-thinking` MCP format
5. Export traces in JSON, JSONL, and Markdown
6. Ship with full test suite, README, and MCP client configuration examples

Detailed version: `contract/goals.md`

## Success Criteria

- All 6 MCP tools registered and functional
- 12 thought kinds, 7 thought statuses, 5 branch statuses, 5 risk levels, 10 risk categories implemented
- Budget enforcement for thoughts (32), branches (8), revisions (12), content (4000 chars)
- Secret redaction for 10+ patterns enabled by default
- TTL expiration and opportunistic cleanup working
- JSON/JSONL/Markdown export returning correct output
- Full test suite passing all categories: compatibility, sessions, branches, revisions, dependencies, budgets, redaction, retention, export, MCP contract
- Cargo.toml with pinned dependency versions (no wildcards)

Detailed version: `contract/success-criteria.md`

## Scope Boundaries

**In scope:** stdio MCP server, all 6 tools, typed thought schema, branching, revisions, dependencies, evidence references, risk classification, action proposals, budget enforcement, TTL retention, secret redaction, metadata-only logging, legacy compatibility, JSON/JSONL/Markdown export, structured errors, config file, env vars, CLI flags, test suite, README

**Out of scope:** executing shell commands, reading/writing local files, browsing the web, calling external APIs, sending emails, mutating external systems, permanent memory, replacing model reasoning, guaranteeing correctness, storing raw private chain-of-thought

Detailed version: `contract/scope.md`

## Constraints

- Rust language with pinned dependency versions (no wildcards in Cargo.toml)
- stdio transport only (no HTTP, SSE, WebSocket)
- In-memory store via `Arc<RwLock<HashMap<...>>>` (no persistence)
- Context7 development workflow required for crate API lookups
- Default privacy: ephemeral, TTL-enabled, redacted, no raw content logs

Detailed version: `contract/constraints.md`

## Assumptions

Detailed version: `contract/assumptions.md`

## Decisions

Detailed version: `contract/decisions.md`

## Risks

Detailed version: `contract/risks.md`

## Approval

- Status: Approved
- Approved By: user
- Approved At: 2026-06-12
