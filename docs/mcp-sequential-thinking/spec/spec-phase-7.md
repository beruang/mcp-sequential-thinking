# Spec Phase 7: Test Suite & Documentation

## Phase Goal

Write the complete test suite (10 categories) and project documentation (README, LICENSE). Validate that all tests pass and documentation covers all required sections.

## Dependencies

- Requires: Phase 6 (working MCP server)
- Produces: `tests/*.rs` (10 files), `README.md`, `LICENSE`

## Existing Code References

- Pattern to follow: Rust integration test pattern — each `tests/*.rs` is a separate crate
- Related module: All source modules (tests exercise the full system)
- Test pattern: `tests/mcp_contract.rs` starts the server as a subprocess and sends MCP JSON-RPC

## Technical Approach

### Test categories

Each test file corresponds to a spec section:

1. **tests/compatibility.rs** — Legacy input format mapping
2. **tests/sessions.rs** — Session CRUD, TTL, cleanup
3. **tests/branches.rs** — Branch creation, validation, status summaries
4. **tests/revisions.rs** — Revision tracking, auto-supersede
5. **tests/dependencies.rs** — Dependency validation (self, future, missing, dedup)
6. **tests/budgets.rs** — All 7 budget types enforced
7. **tests/redaction.rs** — 10+ secret patterns redacted
8. **tests/retention.rs** — TTL expiration and cleanup
9. **tests/export.rs** — JSON, JSONL, Markdown export
10. **tests/mcp_contract.rs** — Tool registration, schemas, annotations, stdio

### Test structure

For lib-level tests (tests that exercise the store, validation, tools directly):
```rust
// tests/sessions.rs
use mcp_sequential_thinking::store::MemoryStore;
use mcp_sequential_thinking::store::ThinkingStore;

#[test]
fn test_new_session_creation() {
    let store = MemoryStore::new(3600);
    let thought = ThoughtInput { ... };
    let result = store.upsert_thought(thought).unwrap();
    assert_eq!(result.thought_number, 1);
}

#[test]
fn test_session_retrieval() { ... }
#[test]
fn test_session_listing() { ... }
#[test]
fn test_session_clearing() { ... }
#[test]
fn test_idempotent_clearing() { ... }
#[test]
fn test_ttl_expiration() { ... }
```

For MCP contract tests (tests that need the running server):
```rust
// tests/mcp_contract.rs
use std::process::{Command, Stdio};
use std::io::Write;

#[test]
fn test_server_starts_and_lists_tools() {
    let mut child = Command::new("./target/debug/mcp-sequential-thinking")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    // Send initialize request
    // Read capabilities response
    // Send tools/list request
    // Verify 6 tools with correct schemas
}
```

### README.md

Must include per spec section 37:
- Project purpose and why structured thinking is useful
- Privacy warning (default: memory only, TTL, no raw content logs)
- Installation instructions (cargo install)
- MCP client config examples (basic, with config file, with Context7)
- Context7 development recommendation
- Tool list with descriptions
- Schema examples for sequentialthinking input/output
- Legacy compatibility mode documentation
- Configuration sections (config file, env vars, CLI flags)
- Security model summary
- Non-goals list
- Test instructions

## File Changes

### New Files

| File | Purpose |
|---|---|
| `tests/compatibility.rs` | Legacy input tests |
| `tests/sessions.rs` | Session lifecycle tests |
| `tests/branches.rs` | Branch validation tests |
| `tests/revisions.rs` | Revision tracking tests |
| `tests/dependencies.rs` | Dependency validation tests |
| `tests/budgets.rs` | Budget enforcement tests |
| `tests/redaction.rs` | Secret redaction tests |
| `tests/retention.rs` | TTL and cleanup tests |
| `tests/export.rs` | Export format tests |
| `tests/mcp_contract.rs` | MCP protocol contract tests |
| `README.md` | Full project documentation |
| `LICENSE` | License file (MIT or Apache 2.0) |

## Implementation Steps

1. Write test helpers (create_test_store, valid_thought_input, etc.)
2. Write `tests/compatibility.rs` — per spec section 35
3. Write `tests/sessions.rs` — per spec section 35
4. Write `tests/branches.rs` — per spec section 35
5. Write `tests/revisions.rs` — per spec section 35
6. Write `tests/dependencies.rs` — per spec section 35
7. Write `tests/budgets.rs` — per spec section 35
8. Write `tests/redaction.rs` — per spec section 35
9. Write `tests/retention.rs` — per spec section 35
10. Write `tests/export.rs` — per spec section 35
11. Write `tests/mcp_contract.rs` — per spec section 35
12. Write `README.md` — per spec section 37
13. Write `LICENSE`
14. Run full test suite, fix failures
15. Run clippy and fmt, fix warnings
16. Verify Cargo.toml has no wildcard versions

## Testing Requirements

### All tests must pass

```bash
cargo test                          # All tests
```

Each test category must cover the cases listed in spec section 35.

## Validation Commands

```bash
cargo test                           # Inner loop: all tests
cargo test --test compatibility      # Specific test file
cargo clippy -- -D warnings         # Lint check
cargo fmt -- --check                 # Format check
grep -r '\*' Cargo.toml | wc -l     # Check for wildcard versions (should be 0)
```

## Acceptance Criteria

- [ ] All 10 test files exist and pass
- [ ] `tests/compatibility.rs` verifies legacy input mapping
- [ ] `tests/sessions.rs` verifies session CRUD and TTL
- [ ] `tests/branches.rs` verifies branch validation
- [ ] `tests/revisions.rs` verifies revision tracking and auto-supersede
- [ ] `tests/dependencies.rs` verifies dependency rules
- [ ] `tests/budgets.rs` verifies all 7 budget types
- [ ] `tests/redaction.rs` verifies 10+ patterns
- [ ] `tests/retention.rs` verifies TTL cleanup
- [ ] `tests/export.rs` verifies JSON, JSONL, Markdown
- [ ] `tests/mcp_contract.rs` verifies tool registration and schemas
- [ ] `README.md` includes all required sections per spec section 37
- [ ] `LICENSE` file exists
- [ ] `cargo clippy` passes with no warnings
- [ ] `cargo fmt` passes
- [ ] Cargo.toml has zero wildcard dependencies
- [ ] `cargo test` exits 0

## Risks

| Risk | Severity | Mitigation |
|---|---|---|
| MCP contract tests are flaky due to subprocess timing | Medium | Use retry logic with timeout for process startup |
| Test coverage gaps in edge cases | Medium | Cross-reference tests against spec section 35 test requirements |
| README becomes stale vs actual behavior | Low | README references the config and spec; not duplicated details |
