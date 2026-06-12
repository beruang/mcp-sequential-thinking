# MCP Sequential Thinking

> A structured reasoning trace MCP server for AI agents. Records typed thinking steps with branching, revision tracking, dependency chains, evidence references, risk classification, and action proposals.

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.74%2B-orange.svg)](https://www.rust-lang.org)
[![MCP](https://img.shields.io/badge/MCP-2025--06--18-blueviolet)](https://modelcontextprotocol.io)

## Overview

`mcp-sequential-thinking` exposes a single MCP server that captures the *shape* of an agent's reasoning, not just the final answer. Every thought is typed (observation, assumption, decision, revision, risk, ...), traceable (branches, dependencies, revisions), and bounded (per-session budgets, TTL, content limits). Traces are exportable as JSON, JSONL, or Markdown for post-hoc review.

Built for agents that need to be auditable, revisable, and machine-actionable.

## Why structured thinking

Free-form thought logs lose context. A typed trace lets a reviewer see:

- which **observations** informed a **decision** (dependency chain),
- which **assumptions** were **revised** and why (revision history),
- which **branches** were explored and which were abandoned (branch provenance),
- which **actions** were proposed and which carried risk (action proposals),
- which **evidence** was consulted (URLs, quotes, tool results).

## Features

- **12 typed thought kinds** — observation, assumption, constraint, risk, option, decision, revision, validation, next_action, blocker, question, final_decision
- **Branching** with full provenance (branch-from, branch-label, branch-status)
- **Revision tracking** with configurable per-session limits
- **Dependency chains** between thoughts
- **Evidence references** — URL, quote, metadata, evidence type
- **Risk classification** — level, category, requires-confirmation
- **Action proposals** — tool, args, requires-approval
- **Session budgets** — max thoughts, branches, revisions, content chars, reason-summary chars
- **TTL-based retention** — sessions expire (default 1 hour), no disk persistence
- **Secret redaction** — API keys, tokens, passwords, JWTs redacted before storage and logging
- **Metadata-only logging** — raw thought content is never logged
- **Export formats** — JSON, JSONL, Markdown
- **Legacy compatibility mode** — accepts the original Sequential Thinking input shape
- **No external calls** — the server does not execute tools, fetch URLs, or call APIs

## Privacy

This server stores structured reasoning traces. Traces may contain sensitive task, user, or project information. By default, traces are in-memory only, expire after 1 hour, and raw thought content is not logged. Secret redaction is enabled by default. See [Security model](#security-model).

## Installation

### From source

```bash
cargo install --path .
```

### Release build

```bash
cargo build --release
# binary at target/release/mcp-sequential-thinking
```

## MCP client configuration

### Basic

```json
{
  "mcpServers": {
    "sequential-thinking": {
      "command": "/usr/local/bin/mcp-sequential-thinking",
      "args": []
    }
  }
}
```

### With config file

```json
{
  "mcpServers": {
    "sequential-thinking": {
      "command": "/usr/local/bin/mcp-sequential-thinking",
      "args": [
        "--config",
        "/Users/you/.config/mcp-sequential-thinking/config.json"
      ]
    }
  }
}
```

### With Context7 (recommended during development)

```json
{
  "mcpServers": {
    "sequential-thinking": {
      "command": "/usr/local/bin/mcp-sequential-thinking",
      "args": []
    },
    "context7": {
      "url": "https://mcp.context7.com/mcp",
      "headers": {
        "CONTEXT7_API_KEY": "${CONTEXT7_API_KEY}"
      }
    }
  }
}
```

When developing this project with an AI coding agent, use the Context7 MCP to fetch current crate documentation (especially `rmcp`) before generating code.

## Tools

| Tool | Description |
|---|---|
| `sequentialthinking` | Record one structured thought in a reasoning session |
| `sequential_thinking` | Alias for `sequentialthinking` |
| `get_thinking_session` | Return one session by ID |
| `list_thinking_sessions` | List active sessions |
| `clear_thinking_session` | Remove one session (idempotent) |
| `export_thinking_session` | Export a session in JSON, JSONL, or Markdown |

## Schema

### sequentialthinking input

```json
{
  "sessionId": "task-001",
  "kind": "assumption",
  "content": "The implementation should use rmcp.",
  "thoughtNumber": 1,
  "totalThoughts": 6,
  "nextThoughtNeeded": true,
  "status": "unverified",
  "confidence": 0.7,
  "branchId": "main",
  "evidence": [],
  "risk": {
    "level": "low",
    "category": "operational",
    "requiresConfirmation": false,
    "description": "SDK API may have changed."
  },
  "tags": ["rust", "mcp"]
}
```

### sequentialthinking output

```json
{
  "sessionId": "task-001",
  "thoughtId": "th_000001",
  "thoughtNumber": 1,
  "totalThoughts": 6,
  "nextThoughtNeeded": true,
  "accepted": true,
  "sessionStatus": "active",
  "branches": [{"branchId": "main", "branchLabel": "main", "status": "active", "thoughtCount": 1}],
  "thoughtHistoryLength": 1,
  "budget": {
    "maxThoughts": 32,
    "thoughtsUsed": 1,
    "thoughtsRemaining": 31,
    "maxBranches": 8,
    "branchesUsed": 1,
    "branchesRemaining": 7,
    "maxRevisions": 12,
    "revisionsUsed": 0,
    "revisionsRemaining": 12
  },
  "redactions": [],
  "warnings": []
}
```

## Thought kinds

| Kind | Meaning |
|---|---|
| `observation` | A fact, finding, or external result |
| `assumption` | A claim not yet verified |
| `constraint` | A requirement, rule, or limitation |
| `risk` | A possible failure mode |
| `option` | A possible approach |
| `decision` | A selected approach |
| `revision` | A correction of earlier reasoning |
| `validation` | A check against assumptions or requirements |
| `next_action` | A proposed next step |
| `blocker` | Something preventing progress |
| `question` | A question that needs resolution |
| `final_decision` | Terminal reasoning summary |

## Legacy compatibility mode

Accepts the original Sequential Thinking input format:

```json
{
  "thought": "We need to inspect the problem first.",
  "thoughtNumber": 1,
  "totalThoughts": 5,
  "nextThoughtNeeded": true
}
```

The `thought` field is mapped to `content`, with `kind` defaulting to `observation` and `sessionId` defaulting to `default`. A `legacy_input` warning is included in the response.

## Configuration

### Config file

```json
{
  "server": {"name": "mcp-sequential-thinking", "transport": "stdio"},
  "compatibility": {"acceptLegacyThoughtField": true, "defaultLegacyKind": "observation", "defaultSessionId": "default"},
  "budgets": {"maxThoughtsPerSession": 32, "maxBranchesPerSession": 8, "maxRevisionsPerSession": 12, "maxContentChars": 4000, "maxReasonSummaryChars": 1000, "maxEvidenceItemsPerThought": 10, "maxTagsPerThought": 20},
  "retention": {"mode": "ephemeral", "ttlSeconds": 3600, "persist": false},
  "logging": {"enabled": true, "logRawContent": false},
  "redaction": {"enabled": true, "redactBeforeStore": true, "redactBeforeLog": true},
  "behavior": {"autoSupersedeRevisedThoughts": true, "allowNonContiguousThoughtNumbers": true, "allowFinalDecisionAfterBudgetExceeded": true}
}
```

### Environment variables

```bash
SEQUENTIAL_THINKING_CONFIG=/path/to/config.json
SEQUENTIAL_THINKING_TTL_SECONDS=3600
SEQUENTIAL_THINKING_MAX_THOUGHTS=32
SEQUENTIAL_THINKING_REDACTION=true
DISABLE_THOUGHT_LOGGING=true
RUST_LOG=info
```

### CLI flags

```bash
mcp-sequential-thinking --config ~/.config/mcp-sequential-thinking/config.json
mcp-sequential-thinking --max-thoughts 64
mcp-sequential-thinking --ttl-seconds 7200
mcp-sequential-thinking --disable-logging
mcp-sequential-thinking --disable-redaction
mcp-sequential-thinking --compatibility-mode
mcp-sequential-thinking --log-level debug
```

## Security model

- **Memory only** — traces are stored in-memory, not persisted to disk
- **TTL** — sessions expire and are cleaned up after the configured TTL (default 1 hour)
- **Redaction** — secrets (API keys, tokens, passwords, JWTs) are redacted before storage and logging
- **Metadata-only logging** — raw thought content is never logged
- **No external calls** — the server does not execute tools, fetch URLs, or call external APIs
- **No file access** — export returns content to the client only; no files are written

## Non-goals

- Executing shell commands
- Reading or writing local files
- Browsing the web or calling external APIs
- Acting as permanent memory
- Replacing model reasoning
- Guaranteeing correctness

## Development

### Prerequisites

- Rust 1.74 or later (stable)
- `cargo`, `rustfmt`, `clippy` (installed via `rustup component add rustfmt clippy`)
- `pre-commit` (optional, for local hooks) — `pip install pre-commit`

### Build and test

```bash
cargo build                   # debug build
cargo build --release         # release build
cargo test                    # run all 88 tests across 14 modules
cargo fmt -- --check          # formatting check
cargo clippy --all-targets -- -D warnings   # lint (deny warnings)
```

### Pre-commit hooks

This repo ships a [pre-commit.com](https://pre-commit.com) config that runs `cargo fmt --check` and `cargo clippy -- -D warnings` on every commit. One-time setup:

```bash
pip install pre-commit
pre-commit install
```

After that, every commit automatically runs the lint chain. CI runs the same checks via `.github/workflows/ci.yml`.

### Project layout

```
src/
  main.rs              # binary entry point
  lib.rs               # library exports
  server.rs            # MCP server setup
  config.rs            # config + CLI parsing
  error.rs             # error types
  export/              # JSON / JSONL / Markdown exporters
  model/               # ThoughtKind, ThoughtRecord, session, branch, risk, evidence, action
  redaction/           # secret redaction
  store/               # in-memory ThinkingStore
  tools/               # MCP tool handlers
  validation/          # input rules and warnings
tests/                 # 14 integration test modules
```

## Contributing

1. Fork the repository and create a feature branch.
2. Run `cargo fmt`, `cargo clippy --all-targets -- -D warnings`, and `cargo test` locally — all must pass.
3. Open a pull request with a clear description of the change and its motivation.

Issues and PRs welcome.

## License

[MIT](LICENSE) — Copyright (c) 2026 mcp-sequential-thinking contributors.
