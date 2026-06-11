# Spec Phase 2: In-Memory Store with Retention

## Phase Goal

Implement the `ThinkingStore` trait and an in-memory `HashMap`-backed store with TTL-based retention and opportunistic cleanup.

## Dependencies

- Requires: Phase 1 (data model types, error types)
- Produces: `src/store/mod.rs`, `src/store/memory.rs`, `src/store/retention.rs`

## Existing Code References

- Pattern to follow: Standard Rust trait-based storage abstraction
- Related module: `src/model/session.rs` (ThinkingSession struct), `src/error.rs` (ThinkingError)
- Test pattern: Unit tests with `#[cfg(test)]` creating a store, inserting thoughts, and verifying retrieval

## Technical Approach

### ThinkingStore trait

```rust
pub trait ThinkingStore: Send + Sync + 'static {
    fn upsert_thought(&self, thought: ThoughtRecord) -> Result<ThoughtRecord, ThinkingError>;
    fn get_session(&self, session_id: &str) -> Result<Option<ThinkingSession>, ThinkingError>;
    fn list_sessions(&self, limit: usize) -> Result<Vec<SessionSummary>, ThinkingError>;
    fn clear_session(&self, session_id: &str) -> Result<bool, ThinkingError>;
    fn cleanup_expired(&self) -> Result<usize, ThinkingError>;
}
```

### MemoryStore

```rust
pub struct MemoryStore {
    sessions: Arc<RwLock<HashMap<String, ThinkingSession>>>,
    ttl_seconds: u64,
}
```

Key behaviors:
- `upsert_thought`: If session doesn't exist, create it. Append thought to session. Update timestamps. Return thought record.
- `get_session`: Return session if it exists and hasn't expired. Return `None` if missing or expired.
- `list_sessions`: Return non-expired sessions sorted by updated_at descending, up to limit.
- `clear_session`: Remove session. Return true if it existed, false if not (idempotent).
- `cleanup_expired`: Remove all sessions where `updated_at + ttl < now`. Return count of removed sessions.

### Retention

- Default TTL: 3600 seconds (configurable)
- Supported modes: `ephemeral` (TTL enforced), `disabled` (no TTL)
- Cleanup runs opportunistically before every `upsert_thought` call
- Expired sessions are dropped without warning
- `SessionStatus::Expired` is set when a session is found to be expired on read
- If retention mode is `disabled`, skip all TTL checks

## File Changes

### New Files

| File | Purpose |
|---|---|
| `src/store/mod.rs` | ThinkingStore trait definition |
| `src/store/memory.rs` | MemoryStore implementation |
| `src/store/retention.rs` | TTL cleanup logic |

## Implementation Steps

1. Define `ThinkingStore` trait in `src/store/mod.rs`
2. Implement `MemoryStore::new(ttl_seconds)` constructor
3. Implement `upsert_thought` — create session if new, append thought, update timestamps
4. Implement `get_session` — check TTL, return session or None
5. Implement `list_sessions` — filter expired, sort, limit
6. Implement `clear_session` — remove from map, return bool
7. Implement `cleanup_expired` — iterate, remove expired, return count
8. Add opportunistic cleanup call in `upsert_thought` before processing
9. Write unit tests

## Data / API / Interface Contract

- All methods are `&self` (no `&mut self` needed due to `Arc<RwLock<...>>`)
- `upsert_thought` takes ownership of `ThoughtRecord`, returns it back (potentially modified with redactions)
- `get_session` returns `Option<ThinkingSession>` — `None` means not found or expired
- `clear_session` returns `bool` — `true` if session existed and was removed
- `cleanup_expired` returns `usize` — count of removed sessions

## Error Handling

- `session_not_found` — session doesn't exist in get/clear/export
- `retention_expired` — session exists but has expired (returned as None, not error)
- `internal_error` — RwLock poison or other internal failure

## Observability

- Logs: `tracing::debug!("cleanup_expired removed {} sessions", count)` (metadata only)
- Metrics: None
- Traces: None (store is internal)
- Alerts: None

## Testing Requirements

### Unit Tests

- Create store, upsert thought, verify retrieval
- Get non-existent session returns None
- List sessions returns empty for new store
- List sessions respects limit
- Clear existing session returns true
- Clear non-existent session returns false (idempotent)
- TTL expiration: insert, manipulate time, verify None
- Cleanup removes expired sessions
- Cleanup returns correct count
- Disabled retention skips TTL checks
- Multiple sessions in list are sorted by updated_at

## Validation Commands

```bash
cargo test --lib store            # Inner loop: store tests
cargo build                        # Compile check
```

## Acceptance Criteria

- [ ] `ThinkingStore` trait defined with all 5 methods
- [ ] `MemoryStore` implements `ThinkingStore`
- [ ] `upsert_thought` creates session on first thought, appends on subsequent
- [ ] `get_session` returns None for expired sessions
- [ ] `list_sessions` filters expired and respects limit
- [ ] `clear_session` is idempotent
- [ ] `cleanup_expired` runs opportunistically and removes expired sessions
- [ ] Retention mode `disabled` skips all TTL checks
- [ ] All unit tests pass

## Risks

| Risk | Severity | Mitigation |
|---|---|---|
| RwLock write contention if MCP client pipelines requests | Low | stdio is single-client; pipelining is rare |
| TTL race (session expires between list and get) | Low | Accept race window; TTL is best-effort |
| HashMap growth unbounded without cleanup | Medium | Opportunistic cleanup on every upsert keeps map bounded |
