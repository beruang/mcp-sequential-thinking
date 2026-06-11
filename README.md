# mcp-sequential-thinking

A structured reasoning trace MCP server for AI agents. Records typed thinking steps with branching, revision tracking, dependency chains, evidence references, risk classification, and action proposals.

## Why structured thinking is useful

Free-form thought logs lose context. `mcp-sequential-thinking` provides typed thought kinds (observation, assumption, decision, revision, etc.), branching for alternatives, dependency tracking, budget enforcement, and exportable traces. AI agents performing complex multi-step tasks benefit from structured reasoning that is auditable, revisable, and machine-actionable.

## Privacy warning

This server stores structured reasoning traces. These traces may contain sensitive task, user, or project information. By default, traces are in-memory only, expire after 1 hour (TTL), and raw thought content is not logged. Secret redaction is enabled by default.

## Installation

```bash
cargo install --path .
```

Or build from source:

```bash
cargo build --release
```

## MCP Client Configuration

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

### With Context7 (development)

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

## Context7 development recommendation

When developing this project with an AI coding agent, use Context7 MCP to fetch current crate documentation, especially for rmcp, before generating code.

## Tools

| Tool | Description |
|---|---|
| `sequentialthinking` | Record one structured thought in a reasoning session |
| `sequential_thinking` | Alias for sequentialthinking |
| `get_thinking_session` | Return one session by ID |
| `list_thinking_sessions` | List active sessions |
| `clear_thinking_session` | Remove one session (idempotent) |
| `export_thinking_session` | Export a session in JSON, JSONL, or Markdown |

## Schema examples

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

- **Memory only**: Traces are stored in-memory, not persisted to disk
- **TTL**: Sessions expire and are cleaned up after the configured TTL (default 1 hour)
- **Redaction**: Secrets (API keys, tokens, passwords, JWTs) are redacted before storage and logging
- **Metadata-only logging**: Raw thought content is never logged
- **No external calls**: The server does not execute tools, fetch URLs, or call external APIs
- **No file access**: Export returns content to the client only; no files are written

## Non-goals

- Executing shell commands
- Reading or writing local files
- Browsing the web or calling external APIs
- Acting as permanent memory
- Replacing model reasoning
- Guaranteeing correctness

## Test instructions

```bash
cargo test                    # Run all tests (62 tests, 10 categories)
cargo clippy -- -D warnings   # Lint
cargo fmt -- --check           # Format check
cargo build --release          # Release build
```
