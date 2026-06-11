# Spec Phase 1: Foundation

## Phase Goal

Create the Rust project skeleton with all data model types, configuration loading, and error types. Every other phase depends on the types and traits defined here.

## Dependencies

- Requires: None
- Produces: `Cargo.toml`, `src/config.rs`, `src/error.rs`, `src/model/*.rs`, `.gitignore`, `src/main.rs` (stub)

## Existing Code References

- Pattern to follow: Standard Rust library crate layout
- Related module: None (greenfield)
- Test pattern: `#[cfg(test)] mod tests` within each source file
- Config pattern: clap derive for CLI, serde for config file, env var overrides

## Technical Approach

### Cargo.toml

Pin all dependency versions using exact versions (no `^`, `~`, `*`). Use Context7 to determine current versions for rmcp, tokio, and other key crates.

```toml
[package]
name = "mcp-sequential-thinking"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "mcp-sequential-thinking"
path = "src/main.rs"

[dependencies]
rmcp = "0.4.0"           # Verify via Context7
tokio = { version = "1.42.0", features = ["full"] }
serde = { version = "1.0.210", features = ["derive"] }
serde_json = "1.0.130"
schemars = "0.8.21"
thiserror = "2.0.3"
anyhow = "1.0.90"
clap = { version = "4.5.20", features = ["derive"] }
tracing = "0.1.40"
tracing-subscriber = "0.3.18"
chrono = { version = "0.4.38", features = ["serde"] }
uuid = { version = "1.10.0", features = ["v4", "serde"] }
regex = "1.11.0"
indexmap = "2.6.0"
```

### Data model types

Implement these enums with `#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]` and `#[serde(rename_all = "snake_case")]`:

- `ThoughtKind` — 12 variants (Observation, Assumption, Constraint, Risk, Option, Decision, Revision, Validation, NextAction, Blocker, Question, FinalDecision)
- `ThoughtStatus` — 7 variants (Unverified, Verified, Rejected, Superseded, Active, Done, Blocked)
- `BranchStatus` — 5 variants (Active, Selected, Rejected, Merged, Superseded)
- `RiskLevel` — 5 variants (None, Low, Medium, High, Critical)
- `RiskCategory` — 10 variants (Privacy, Security, DestructiveAction, ExternalSideEffect, DataLoss, CredentialExposure, Cost, Legal, Operational, Unknown)
- `SessionStatus` — 4 variants (Active, Completed, Expired, Cleared)
- `EvidenceType` — 8 variants (ToolResult, File, Url, McpResource, UserMessage, AssistantMessage, ManualNote, Citation)
- `ExportFormat` — 3 variants (Json, Jsonl, Markdown)
- `RetentionMode` — 2 variants (Ephemeral, Disabled)

Implement these structs with `#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]` and `#[serde(rename_all = "camelCase")]`:

- `ThoughtRecord` — full thought with all fields per spec section 29
- `ThinkingSession` — session with thoughts vector and branch summaries
- `EvidenceRef` — evidence reference (type, source, title, uri, reference, quote, metadata)
- `RiskInfo` — risk classification (level, category, requiresConfirmation, description)
- `ActionProposal` — proposed action (tool, args, risk, requiresApproval)
- `BranchSummary` — branch metadata (branchId, branchLabel, branchFromThought, branchStatus, thoughtCount)
- `SessionSummary` — session metadata (sessionId, status, createdAt, updatedAt, thoughtCount, branchCount)
- `RedactionSummary` — redaction result (kind, count)
- `BudgetState` — budget tracking (max/used/remaining for thoughts, branches, revisions)
- `Warning` — warning with code and message

### Config types

Implement `AppConfig` struct with sections for server, compatibility, budgets, retention, logging, redaction, behavior. Load from JSON file, then overlay env vars, then overlay CLI flags.

### Error types

Use `thiserror` for internal errors and define an `ErrorCode` enum with all 15 error codes. Implement conversion to MCP-compatible error JSON.

## File Changes

### New Files

| File | Purpose |
|---|---|
| `Cargo.toml` | Pinned dependencies, package metadata |
| `.gitignore` | Rust standard ignores |
| `src/main.rs` | Binary entrypoint (stub: prints "mcp-sequential-thinking" and exits) |
| `src/config.rs` | Config loading from file/env/CLI with clap |
| `src/error.rs` | Error types, ErrorCode enum, MCP error conversion |
| `src/model/mod.rs` | Module declarations |
| `src/model/session.rs` | ThinkingSession, SessionStatus, SessionSummary |
| `src/model/thought.rs` | ThoughtRecord, ThoughtKind, ThoughtStatus |
| `src/model/branch.rs` | BranchSummary, BranchStatus |
| `src/model/evidence.rs` | EvidenceRef, EvidenceType |
| `src/model/risk.rs` | RiskInfo, RiskLevel, RiskCategory |
| `src/model/action.rs` | ActionProposal |
| `src/model/budget.rs` | BudgetState |
| `src/model/redaction.rs` | RedactionSummary |
| `src/model/warning.rs` | Warning |

## Implementation Steps

1. Run `cargo init` in the project directory (if not already a cargo project)
2. Use Context7 MCP to look up current rmcp, tokio, schemars, clap versions
3. Write `Cargo.toml` with pinned versions
4. Write `src/model/mod.rs` declaring all submodules
5. Implement `src/model/session.rs` — SessionStatus enum, ThinkingSession struct, SessionSummary struct
6. Implement `src/model/thought.rs` — ThoughtKind enum (12 variants), ThoughtStatus enum (7 variants), ThoughtRecord struct
7. Implement `src/model/branch.rs` — BranchStatus enum (5 variants), BranchSummary struct
8. Implement `src/model/evidence.rs` — EvidenceType enum (8 variants), EvidenceRef struct
9. Implement `src/model/risk.rs` — RiskLevel enum (5 variants), RiskCategory enum (10 variants), RiskInfo struct
10. Implement `src/model/action.rs` — ActionProposal struct
11. Implement `src/model/budget.rs` — BudgetState struct
12. Implement `src/model/redaction.rs` — RedactionSummary struct
13. Implement `src/model/warning.rs` — Warning struct
14. Implement `src/error.rs` — ErrorCode enum (15 variants), ThinkingError struct with thiserror
15. Implement `src/config.rs` — AppConfig struct, load from file, env vars, CLI via clap derive
16. Write `src/main.rs` stub
17. Write `.gitignore` (target/, .env, *.swp, etc.)
18. Run `cargo build` and `cargo check` to verify

## Data / API / Interface Contract

- All enums serialize to `snake_case` JSON strings
- All struct fields serialize to `camelCase` JSON keys
- `ThoughtRecord.thought_id` is generated as `th_` + zero-padded incrementing number
- `ThinkingSession.session_id` is client-provided
- `chrono::DateTime<chrono::Utc>` serializes to ISO 8601 strings
- `Option<T>` fields omit `null` in JSON when `#[serde(skip_serializing_if = "Option::is_none")]`

## Error Handling

- `ThinkingError` enum with variants for each error code
- `to_mcp_error()` method produces `{"error": {"code": "...", "message": "..."}}` JSON
- Field-level errors include `"field"` key
- Internal errors produce `internal_error` code with sanitized message (no stack traces)

## Observability

- Logs: `tracing` instrumented with metadata only (session ID, thought number, kind, status)
- Metrics: None (stdio server, metrics not required)
- Traces: Structured thought records are the trace
- Alerts: None

## Testing Requirements

### Unit Tests

- Each enum round-trips through serde correctly
- `ThoughtRecord` JSON matches spec examples
- `ThinkingSession` JSON matches spec examples
- Config loads from JSON file correctly
- Config overrides from env vars correctly
- CLI flags override config correctly

## Validation Commands

```bash
cargo build                     # Inner loop: compile
cargo test --lib                # Run unit tests
cargo clippy -- -D warnings    # Lint
```

## Acceptance Criteria

- [ ] `Cargo.toml` exists with no wildcard versions
- [ ] `cargo build` compiles without errors
- [ ] All 9 enums defined with correct variants and serde rename
- [ ] All 6 structs defined with correct fields and camelCase rename
- [ ] `ThoughtKind::Observation` serializes to `"observation"`
- [ ] `ThoughtStatus::Unverified` serializes to `"unverified"`
- [ ] `BranchStatus::Active` serializes to `"active"`
- [ ] `SessionStatus::Active` serializes to `"active"`
- [ ] `RiskLevel::Low` serializes to `"low"`
- [ ] `RiskCategory::Operational` serializes to `"operational"`
- [ ] Config loads from file path, env vars, and CLI flags with correct precedence
- [ ] All 15 error codes defined in ErrorCode enum
- [ ] `.gitignore` covers `target/`

## Risks

| Risk | Severity | Mitigation |
|---|---|---|
| rmcp version incompatible with tokio version | Medium | Pin both to versions known to work together via Context7 |
| schemars generates schemas that differ from hand-written spec | Low | Verify with `cargo test` comparing generated schema to expected |
| chrono serde format differs from expected ISO 8601 | Low | Use chrono's `serde` feature; verify in tests |
