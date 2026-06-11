# Problem Statement

## Current pain

AI agents performing complex multi-step tasks lack structured reasoning infrastructure. The original Sequential Thinking MCP server only provides a free-form text thought log — no typed kinds, no branching for alternatives, no revision tracking, no dependency chains, no evidence references, and no budget enforcement.

## Affected user/operator/agent

AI coding agents that need to:
- Plan multi-step implementation tasks
- Compare alternative approaches (branching)
- Revise earlier reasoning when new information arrives
- Track dependencies between decisions
- Reference external evidence (tool results, documentation)
- Stay within reasoning budgets
- Export auditable reasoning traces

## Current workaround

Agents use unstructured chain-of-thought in their context window, which is:
- Not auditable after the session ends
- Not structured for machine consumption
- Not budgeted (can consume unlimited context)
- Not revisable without re-reading the entire trace
- Not exportable in structured formats

## Impact

- Agents lose reasoning context between sessions
- No audit trail for decisions made during implementation
- No way to enforce reasoning discipline (budgets, revision tracking)
- Sensitive content may leak into logs without redaction
- Complex tasks require re-reasoning from scratch

## Why now

The MCP ecosystem is maturing. Structured reasoning is the natural next step beyond the original free-form Sequential Thinking. The rmcp Rust SDK provides a stable foundation for building production MCP servers. AI coding agents are tackling increasingly complex tasks that benefit from structured reasoning traces.
