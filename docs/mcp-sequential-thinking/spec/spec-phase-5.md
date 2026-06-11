# Spec Phase 5: Export Formatters

## Phase Goal

Implement JSON, JSONL, and Markdown export formatters for session traces.

## Dependencies

- Requires: Phase 1 (data models only)
- Produces: `src/export/mod.rs`, `src/export/json.rs`, `src/export/jsonl.rs`, `src/export/markdown.rs`

## Existing Code References

- Pattern to follow: Each formatter is a pure function: `fn format_session(session: &ThinkingSession) -> String`
- Related module: `src/model/session.rs` (ThinkingSession), `src/model/thought.rs` (ThoughtRecord)
- Test pattern: Create a ThinkingSession with known thoughts, format, verify output string

## Technical Approach

### JSON export

Returns the session as a pretty-printed JSON object:

```rust
pub fn format_session_json(session: &ThinkingSession) -> Result<String, ThinkingError> {
    let output = serde_json::to_string_pretty(&serde_json::json!({
        "sessionId": session.session_id,
        "status": session.status,
        "createdAt": session.created_at,
        "updatedAt": session.updated_at,
        "thoughtCount": session.thoughts.len(),
        "branchCount": session.branches.len(),
        "branches": session.branches,
        "thoughts": session.thoughts,
    }))?;
    Ok(output)
}
```

### JSONL export

One thought per line, each line is a valid JSON object:

```rust
pub fn format_session_jsonl(session: &ThinkingSession) -> Result<String, ThinkingError> {
    let mut output = String::new();
    for thought in &session.thoughts {
        let line = serde_json::to_string(thought)?;
        output.push_str(&line);
        output.push('\n');
    }
    Ok(output)
}
```

### Markdown export

Human-readable format matching spec section 33:

```rust
pub fn format_session_markdown(session: &ThinkingSession) -> Result<String, ThinkingError> {
    let mut output = String::new();
    output.push_str(&format!("# Thinking Session {}\n\n", session.session_id));
    output.push_str(&format!("Status: {:?}\n", session.status));
    output.push_str(&format!("Thoughts: {}\n", session.thoughts.len()));
    output.push_str(&format!("Branches: {}\n\n", session.branches.len()));

    for thought in &session.thoughts {
        output.push_str(&format!("## Thought {} — {:?}\n\n", thought.thought_number, thought.kind));
        output.push_str(&format!("Status: {:?}\n", thought.status));
        if let Some(conf) = thought.confidence {
            output.push_str(&format!("Confidence: {}\n", conf));
        }
        if let Some(ref summary) = thought.reason_summary {
            output.push_str(&format!("\n{}\n", summary));
        }
        output.push_str(&format!("\n{}\n\n", thought.content));
        // Evidence, risk, action proposal if present
        if !thought.evidence.is_empty() {
            output.push_str("**Evidence:**\n\n");
            for ev in &thought.evidence {
                output.push_str(&format!("- [{}] {} ({})\n", ev.source, ev.title, ev.reference));
            }
            output.push('\n');
        }
        if let Some(ref risk) = thought.risk {
            output.push_str(&format!("**Risk:** {:?} / {:?} — {}\n\n", risk.level, risk.category, risk.description));
        }
        if let Some(ref action) = thought.action_proposal {
            output.push_str(&format!("**Action:** `{}`\n\n", action.tool));
        }
        // Branch/dependency/revision info
        if let Some(ref bid) = thought.branch_id {
            output.push_str(&format!("Branch: {}\n\n", bid));
        }
        if !thought.depends_on.is_empty() {
            output.push_str(&format!("Depends on: {:?}\n\n", thought.depends_on));
        }
        if thought.is_revision {
            if let Some(rev) = thought.revises_thought {
                output.push_str(&format!("Revises thought: {}\n\n", rev));
            }
        }
    }
    Ok(output)
}
```

## File Changes

### New Files

| File | Purpose |
|---|---|
| `src/export/mod.rs` | Module declarations, `format_session()` dispatcher |
| `src/export/json.rs` | JSON formatter |
| `src/export/jsonl.rs` | JSONL formatter |
| `src/export/markdown.rs` | Markdown formatter |

## Implementation Steps

1. Define `ExportFormat` enum in model (already in Phase 1)
2. Implement `format_session_json` — pretty-printed JSON with session metadata and thoughts
3. Implement `format_session_jsonl` — one JSON line per thought
4. Implement `format_session_markdown` — full markdown with headers, metadata, evidence, risk
5. Implement dispatcher function: `format_session(session, format) -> Result<String, ThinkingError>`
6. Write unit tests for each format

## Data / API / Interface Contract

- All formatters take `&ThinkingSession` and return `Result<String, ThinkingError>`
- JSON output is pretty-printed with 2-space indent
- JSONL output has no trailing newline after the last line (but one newline per line is fine)
- Markdown output uses `#` for session title, `##` for each thought
- Empty sessions produce valid output (JSON with empty thoughts array, empty JSONL, markdown with header only)

## Error Handling

- `serialization_error` — serde_json serialization failure
- Errors should include context (which thought failed to serialize in JSONL mode)

## Observability

- Logs: None (pure functions)
- Metrics: None
- Traces: None
- Alerts: None

## Testing Requirements

### Unit Tests

- JSON: session with 2 thoughts → valid JSON with 2 thought objects
- JSON: empty session → valid JSON with empty thoughts array
- JSON: session with branches → branches included
- JSONL: session with 3 thoughts → 3 lines
- JSONL: empty session → empty string
- JSONL: each line is valid JSON (parseable)
- Markdown: session with 1 thought → contains `# Thinking Session`
- Markdown: contains thought kind, status, content
- Markdown: contains branch info when present
- Markdown: contains evidence list when present
- Markdown: contains risk info when present
- Markdown: contains action proposal when present
- Markdown: empty session → header only
- Dispatcher: "json" → calls JSON formatter
- Dispatcher: "jsonl" → calls JSONL formatter
- Dispatcher: "markdown" → calls Markdown formatter
- Dispatcher: invalid format → error

## Validation Commands

```bash
cargo test --lib export            # Inner loop: export tests
cargo build                         # Compile check
```

## Acceptance Criteria

- [ ] JSON export produces valid JSON matching spec section 24.6 examples
- [ ] JSONL export produces one valid JSON object per line
- [ ] Markdown export produces readable formatted output with all thought fields
- [ ] Empty sessions handled correctly in all formats
- [ ] All unit tests pass

## Risks

| Risk | Severity | Mitigation |
|---|---|---|
| Markdown escaping issues (content contains `#` or `**`) | Low | Content is printed as-is after a blank line; markdown headings use `##` prefix |
| Large sessions produce oversized export strings | Low | Budget enforcement (max 32 thoughts) keeps sessions bounded |
