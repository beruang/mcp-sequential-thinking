# Goals

## Primary goals

1. **Structured reasoning server** — Build a Rust MCP server over stdio that records typed thinking steps with full metadata (kind, status, confidence, branch, dependencies, evidence, risk, action proposals).

2. **Complete tool surface** — Expose 6 MCP tools: `sequentialthinking`, `sequential_thinking` (alias), `get_thinking_session`, `list_thinking_sessions`, `clear_thinking_session`, `export_thinking_session`.

3. **Budget enforcement** — Enforce configurable limits on thoughts per session (32), branches (8), revisions (12), content length (4000 chars), evidence items (10), and tags (20).

4. **Privacy by default** — Redact 10+ secret patterns before storage and logging. Log metadata only (never raw thought content). TTL-expire sessions after 1 hour.

5. **Legacy compatibility** — Accept the original Sequential Thinking input format (`thought`, `thoughtNumber`, `totalThoughts`, `nextThoughtNeeded`) and map it to structured fields.

6. **Multi-format export** — Export session traces as JSON, JSONL, and Markdown via the `export_thinking_session` tool.

7. **Comprehensive test suite** — Tests for compatibility, sessions, branches, revisions, dependencies, budgets, redaction, retention, export, and MCP contract compliance.

8. **Pinned dependencies** — Cargo.toml with exact version pins; no wildcard versions.

## Secondary goals

1. Structured MCP-compatible errors with field-level detail
2. Configuration via file, environment variables, and CLI flags with clear precedence
3. Optional `dashmap` support for higher-concurrency stores (not required for initial implementation)
4. Background cleanup task for expired sessions (optional; opportunistic cleanup is sufficient)

## Measurable targets

| Goal | Metric | Target |
|---|---|---|
| Structured reasoning | Thought kinds supported | 12/12 |
| Complete tool surface | Tools registered | 6/6 |
| Budget enforcement | Budget violations caught | All 7 budget types enforced |
| Privacy | Secret patterns redacted | 10+ patterns covered |
| Legacy compatibility | Legacy input accepted | 100% field mapping |
| Export | Formats supported | 3/3 (JSON, JSONL, Markdown) |
| Test suite | Test categories passing | 10/10 categories |
| Dependencies | Wildcard versions | 0 |

## Priority order

1. Structured reasoning server (goals 1, 2)
2. Budget enforcement and privacy (goals 3, 4)
3. Legacy compatibility (goal 5)
4. Export (goal 6)
5. Test suite (goal 7)
6. Pinned dependencies (goal 8)
