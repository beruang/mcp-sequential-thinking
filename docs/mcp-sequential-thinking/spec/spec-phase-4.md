# Spec Phase 4: Tool Handlers

## Phase Goal

Implement handler functions for all 6 MCP tools. Each handler accepts JSON input, calls the store and validation pipeline, and returns JSON output.

## Dependencies

- Requires: Phase 3 (validation pipeline, store)
- Produces: `src/tools/mod.rs`, `src/tools/sequentialthinking.rs`, `src/tools/get_session.rs`, `src/tools/list_sessions.rs`, `src/tools/clear_session.rs`, `src/tools/export_session.rs`

## Existing Code References

- Pattern to follow: Each tool handler is a pure function: `fn handle_*(input: Input, store: &dyn ThinkingStore, config: &AppConfig) -> Result<serde_json::Value, ThinkingError>`
- Related module: `src/validation/mod.rs` (validate_thought), `src/store/mod.rs` (ThinkingStore trait), `src/export/mod.rs` (export formatters)
- Test pattern: Create in-memory store, call handler, verify JSON output against expected

## Technical Approach

### sequentialthinking handler

```rust
pub fn handle_sequentialthinking(
    input: ThoughtInput,
    store: &dyn ThinkingStore,
    config: &AppConfig,
) -> Result<SequentialThinkingOutput, ThinkingError> {
    // 1. Validate through pipeline (includes cleanup, normalization, validation, redaction, budgets)
    // 2. Insert thought into store
    // 3. Update branch summaries
    // 4. Auto-supersede if configured
    // 5. Build response with budget, branches, redactions, warnings
}
```

### sequential_thinking handler

```rust
pub fn handle_sequential_thinking(
    input: ThoughtInput,
    store: &dyn ThinkingStore,
    config: &AppConfig,
) -> Result<SequentialThinkingOutput, ThinkingError> {
    handle_sequentialthinking(input, store, config)  // Same handler
}
```

### get_thinking_session handler

```rust
pub fn handle_get_session(
    input: GetSessionInput,
    store: &dyn ThinkingStore,
) -> Result<GetSessionOutput, ThinkingError> {
    // 1. Call store.get_session(sessionId)
    // 2. If None, return session_not_found error
    // 3. If includeThoughts is false, return metadata only (omit thoughts array)
    // 4. If includeThoughts is true, return full session with thoughts
}
```

### list_thinking_sessions handler

```rust
pub fn handle_list_sessions(
    input: ListSessionsInput,
    store: &dyn ThinkingStore,
) -> Result<ListSessionsOutput, ThinkingError> {
    // 1. Clamp limit to [1, 500]; default 50
    // 2. Call store.list_sessions(limit)
    // 3. Return sessions array
}
```

### clear_thinking_session handler

```rust
pub fn handle_clear_session(
    input: ClearSessionInput,
    store: &dyn ThinkingStore,
) -> Result<ClearSessionOutput, ThinkingError> {
    // 1. Call store.clear_session(sessionId)
    // 2. Return { sessionId, cleared: bool }
    // 3. Idempotent: missing session returns cleared: false (not error)
}
```

### export_thinking_session handler

```rust
pub fn handle_export_session(
    input: ExportSessionInput,
    store: &dyn ThinkingStore,
    config: &AppConfig,
) -> Result<ExportSessionOutput, ThinkingError> {
    // 1. Call store.get_session(sessionId)
    // 2. If None, return session_not_found error
    // 3. Call appropriate export formatter based on format
    // 4. Return formatted content
}
```

### Input/Output types

These are the JSON-serializable types that the MCP layer converts to/from:

```rust
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThoughtInput {
    pub session_id: Option<String>,
    pub kind: Option<String>,
    pub content: Option<String>,
    pub thought: Option<String>,           // Legacy field
    pub thought_number: Option<u32>,
    pub total_thoughts: Option<u32>,
    pub next_thought_needed: Option<bool>,
    pub status: Option<String>,
    pub confidence: Option<f32>,
    pub reason_summary: Option<String>,
    pub branch_id: Option<String>,
    pub branch_label: Option<String>,
    pub branch_from_thought: Option<u32>,
    pub branch_status: Option<String>,
    pub is_revision: Option<bool>,
    pub revises_thought: Option<u32>,
    pub depends_on: Option<Vec<u32>>,
    pub evidence: Option<Vec<EvidenceRef>>,
    pub risk: Option<RiskInfo>,
    pub action_proposal: Option<ActionProposal>,
    pub tags: Option<Vec<String>>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SequentialThinkingOutput {
    pub session_id: String,
    pub thought_id: String,
    pub thought_number: u32,
    pub total_thoughts: u32,
    pub next_thought_needed: bool,
    pub accepted: bool,
    pub session_status: String,
    pub branches: Vec<BranchSummary>,
    pub thought_history_length: usize,
    pub budget: BudgetState,
    pub redactions: Vec<RedactionSummary>,
    pub warnings: Vec<Warning>,
}
```

## File Changes

### New Files

| File | Purpose |
|---|---|
| `src/tools/mod.rs` | Module declarations, shared input/output types |
| `src/tools/sequentialthinking.rs` | sequentialthinking + sequential_thinking handler |
| `src/tools/get_session.rs` | get_thinking_session handler |
| `src/tools/list_sessions.rs` | list_thinking_sessions handler |
| `src/tools/clear_session.rs` | clear_thinking_session handler |
| `src/tools/export_session.rs` | export_thinking_session handler |

## Implementation Steps

1. Define all input/output types in `src/tools/mod.rs`
2. Implement `handle_sequentialthinking` — full validation + store + response building
3. Implement `handle_sequential_thinking` — delegates to sequentialthinking
4. Implement `handle_get_session` — store lookup, includeThoughts filtering
5. Implement `handle_list_sessions` — limit clamping, store listing
6. Implement `handle_clear_session` — store clear, idempotent
7. Implement `handle_export_session` — store lookup + formatter call
8. Add terminal thought warning (nextThoughtNeeded=false, kind != final_decision)
9. Write unit tests for each handler

## Data / API / Interface Contract

- All handlers return `Result<T, ThinkingError>` where T is the output type
- Errors are serialized as `{"error": {"code": "...", "message": "..."}}` by the MCP layer
- `SequentialThinkingOutput.accepted` is always `true` when Ok (false would be error)
- `get_session` with `includeThoughts: false` omits the `thoughts` field entirely
- `list_sessions` default limit: 50, max: 500
- `clear_session` returns `{"sessionId": "...", "cleared": true/false}` — never errors
- `export_session` format defaults to `"json"` if not specified

## Error Handling

Per spec section 24, each handler returns specific error codes:
- sequentialthinking: `invalid_input`, `invalid_kind`, `invalid_status`, `invalid_confidence`, `missing_session_id`, `budget_exceeded`, `content_too_large`, `invalid_branch`, `invalid_revision`, `invalid_dependency`
- get_session: `session_not_found`
- list_sessions: none (always succeeds)
- clear_session: none (always succeeds, returns false for missing)
- export_session: `session_not_found`, `invalid_input` (bad format)

## Observability

- Logs: `tracing::info!(session=%id, tool="sequentialthinking", thought=%n, kind=%k)` (metadata only)
- Metrics: None
- Traces: Each handler returns structured output
- Alerts: None

## Testing Requirements

### Unit Tests

- sequentialthinking: valid input → valid output with budget, branches, no errors
- sequentialthinking: legacy input → output with legacy_input warning
- sequentialthinking: empty sessionId rejected
- sequentialthinking: invalid kind rejected
- sequentialthinking: confidence out of range rejected
- sequentialthinking: budget exceeded rejected
- sequentialthinking: terminal thought with non-final_decision kind → warning
- sequential_thinking: produces same output as sequentialthinking for same input
- get_session: existing session returned with thoughts
- get_session: includeThoughts=false omits thoughts
- get_session: non-existent session → error
- list_sessions: returns sessions up to limit
- list_sessions: limit clamped to 500
- clear_session: existing session → cleared: true
- clear_session: non-existent session → cleared: false (not error)
- export_session: JSON format → structured JSON
- export_session: JSONL format → newline-delimited
- export_session: Markdown format → markdown string
- export_session: non-existent session → error

## Validation Commands

```bash
cargo test --lib tools             # Inner loop: tool handler tests
cargo build                         # Compile check
```

## Acceptance Criteria

- [ ] All 6 tool handlers implemented
- [ ] sequentialthinking validates input through the full pipeline
- [ ] sequential_thinking produces identical output
- [ ] get_session respects includeThoughts flag
- [ ] list_sessions clamps limit to [1, 500]
- [ ] clear_session is idempotent
- [ ] export_session calls correct formatter for each format
- [ ] All unit tests pass

## Risks

| Risk | Severity | Mitigation |
|---|---|---|
| Handler logic diverges from spec output format | Medium | Cross-reference every output field against spec section 24 examples |
| export_session integration with formatters breaks | Low | Phase 5 formatters have their own tests; export handler tests verify integration |
| SequentialThinkingOutput grows too large (many thoughts) | Low | Budget enforcement keeps session sizes bounded |
