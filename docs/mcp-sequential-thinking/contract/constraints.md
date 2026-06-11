# Constraints

## File layout constraints

- Repository root: `mcp-sequential-thinking/`
- Source under `src/` with modules: `model/`, `store/`, `tools/`, `redaction/`, `validation/`, `export/`
- Tests under `tests/` with one file per test category
- Binary entrypoint at `src/main.rs`
- Config module at `src/config.rs`
- Server module at `src/server.rs`
- Error module at `src/error.rs`
- Docs under `docs/mcp-sequential-thinking/`
- Agent artifacts under `.agent/contracts/mcp-sequential-thinking/`

## Naming constraints

- Repository: `mcp-sequential-thinking`
- Rust package: `mcp-sequential-thinking`
- Binary: `mcp-sequential-thinking`
- Rust module: `mcp_sequential_thinking`
- MCP tool: `sequentialthinking`
- Alias tool: `sequential_thinking`
- Serde: `snake_case` for enums, `camelCase` for struct fields

## Codebase constraints

- Greenfield project (no existing code to constrain)
- Must use `Arc<RwLock<HashMap<...>>>` for initial store (dashmap optional)
- Must implement `ThinkingStore` trait for store abstraction
- Validation pipeline must follow 17-step order defined in spec

## Tool constraints

- Required: rmcp, tokio, serde, serde_json, schemars, thiserror, anyhow, clap, tracing, tracing-subscriber, chrono, uuid, regex, indexmap
- Optional: dashmap, insta, tempfile, assert_cmd, predicates
- Context7 MCP must be used before implementing crate API calls
- All dependency versions must be pinned (no `*`, `^`, or `>=` without exact version)

## Context constraints

- Max content per thought: 4000 chars (configurable)
- Max reason summary: 1000 chars (configurable)
- Max evidence items per thought: 10
- Max tags per thought: 20
- Max thoughts per session: 32 (configurable)
- Max branches per session: 8 (configurable)
- Max revisions per session: 12 (configurable)

## Security constraints

- Redact before store (must)
- Redact before log (must)
- No raw content logging (default)
- No persistence by default (ephemeral mode)
- No external API calls
- No filesystem writes (export returns to client only)
- No execution of action proposals
- No fetching or verifying evidence URLs
- TTL expiration enabled by default (3600s)
