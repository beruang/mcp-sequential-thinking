# Decisions

| Decision | Reason | Alternatives considered | Impact |
|---|---|---|---|
| Use rmcp for MCP protocol handling | Official Rust MCP SDK; handles stdio transport, tool registration, and JSON-RPC | Raw JSON-RPC over stdin/stdout (more boilerplate), custom MCP implementation (reinventing wheel) | All tool handlers and server bootstrap depend on rmcp API |
| Use `Arc<RwLock<HashMap<...>>>` for in-memory store | Simple, correct, no extra dependency; acceptable for single-process stdio server | `dashmap` (faster concurrent reads but extra dependency), `Arc<Mutex<...>>` (blocks reads unnecessarily) | Store trait implementation and all tool handlers use this pattern |
| Use ephemeral retention with opportunistic cleanup | Matches privacy requirements; no background task complexity | Background cleanup task (adds complexity for marginal gain), no cleanup (violates TTL requirement) | `cleanup_expired` called at start of every tool invocation |
| Use `snake_case` for serde enum variants, `camelCase` for struct fields | Standard Rust convention for enums; standard JSON convention for fields | All `snake_case` (non-standard JSON), all `camelCase` (non-standard Rust) | Every serialized type must use correct `#[serde(rename_all)]` |
| Default branch is "main" when none provided | Matches the single-reasoning-path default; explicit branching is opt-in | No default branch (would required branchId on every thought), "default" (less conventional) | Branch summary tracking and validation logic |
| `sequential_thinking` is a tool alias, not a separate implementation | Reduces code duplication; both call the same handler | Separate tool with duplicated schema (would drift), client-side alias (not all clients support it) | Tool registration registers both names to the same handler function |
| Content is redacted before storage AND before logging | Defense in depth; secrets should never be stored or logged | Redact only before logging (secrets leak to store), redact only before storage (secrets leak to logs) | Redaction runs at step 8 in the validation pipeline, before both storage and logging |
| `final_decision` is allowed after budget exhaustion if configured | Reasoning should be completable even at budget limit | Hard reject at budget limit (incomplete traces), unlimited budget (context exhaustion) | Budget enforcement has a `allowFinalDecisionAfterBudgetExceeded` config flag |

## Superseded decisions

None — initial contract, no prior decisions.

## Rejected alternatives

| Alternative | Reason rejected |
|---|---|
| HTTP/SSE transport | Spec explicitly requires stdio only for initial implementation |
| Persistent storage (SQLite) | Spec requires ephemeral in-memory only; persistence is a future consideration |
| Dashmap for concurrent store | `Arc<RwLock<HashMap<...>>>` is simpler and sufficient for single-client stdio |
| Background TTL cleanup task | Opportunistic cleanup on every tool call is simpler and sufficient |
| Auto-generating session IDs | Spec requires client-provided session IDs for structured input |
| Combining `sequentialthinking` and `sequential_thinking` into one tool name | Compatibility alias is required by spec for existing clients |
