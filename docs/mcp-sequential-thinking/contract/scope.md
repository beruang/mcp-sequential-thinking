# Scope Boundaries

## In scope

- stdio MCP server binary (`mcp-sequential-thinking`)
- `sequentialthinking` tool with 12 thought kinds, 7 statuses, branching, revisions, dependencies
- `sequential_thinking` alias tool (same handler)
- `get_thinking_session` tool (read session by ID)
- `list_thinking_sessions` tool (list active sessions)
- `clear_thinking_session` tool (remove session, idempotent)
- `export_thinking_session` tool (JSON, JSONL, Markdown)
- In-memory session store via `Arc<RwLock<HashMap<...>>>`
- Typed thought schema with confidence, evidence, risk, action proposals
- Branch lifecycle (active, selected, rejected, merged, superseded)
- Revision tracking with auto-supersede
- Dependency tracking with validation (no self, future, or missing deps)
- Evidence references (8 types: tool_result, file, url, mcp_resource, user_message, assistant_message, manual_note, citation)
- Risk classification (5 levels × 10 categories)
- Action proposals (recorded, never executed)
- Configurable budget enforcement (7 budget types)
- TTL retention with opportunistic cleanup
- Secret redaction (10+ patterns) before storage and logging
- Metadata-only logging (raw content disabled by default)
- Legacy compatibility mode (accept `thought` field)
- Structured MCP-compatible errors (15 error codes)
- Configuration via JSON file, environment variables, CLI flags
- Full test suite (10 categories)
- README with client configuration examples
- Pinned Cargo.toml dependencies
- Context7 development workflow documentation

## Out of scope

- Executing shell commands
- Reading local files
- Writing local files
- Browsing the web
- Calling external APIs
- Sending emails
- Mutating external systems
- Acting as permanent memory (by default)
- Replacing model reasoning
- Guaranteeing correctness
- Storing raw private chain-of-thought (by default)
- HTTP, SSE, or WebSocket transport
- Persistent storage (disk, database)
- Multi-process or distributed operation
- Authentication or authorization
- Rate limiting beyond budget enforcement
- Executing action proposals (recorded only)
- Fetching or verifying evidence references
- Background cleanup task (opportunistic only)

## Future considerations

- Persistent storage backend (SQLite, file-based)
- HTTP/SSE transport for remote MCP clients
- Dashboard or UI for browsing traces
- Session merge and diff operations
- Export to additional formats (HTML, CSV, PDF)
- Background TTL cleanup task
- Dashmap for concurrent access under high load
- Plugin system for custom thought kinds

## Explicitly deferred

None — the spec explicitly defines the complete feature set as single-shot. No features are deferred.
