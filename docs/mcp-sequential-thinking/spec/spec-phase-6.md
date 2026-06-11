# Spec Phase 6: MCP Server Integration

## Phase Goal

Wire all tool handlers into a working rmcp MCP server over stdio. Register all 6 tools with correct JSON schemas and annotations. Implement `main.rs` with CLI flag parsing, config loading, and server bootstrap.

## Dependencies

- Requires: Phase 4 (tool handlers), Phase 5 (export formatters)
- Produces: `src/server.rs`, `src/main.rs` (final version)

## Existing Code References

- Pattern to follow: rmcp stdio server example (fetch via Context7)
- Related module: `src/tools/*.rs` (handlers), `src/config.rs` (AppConfig), `src/store/memory.rs` (MemoryStore)
- Test pattern: Integration test that starts server as subprocess and sends JSON-RPC

## Technical Approach

### server.rs — MCP server setup

```rust
use rmcp::{Server, ServerBuilder, tool, Tool};

pub async fn run_server(config: AppConfig) -> anyhow::Result<()> {
    // Initialize tracing subscriber
    // ...

    let store = Arc::new(MemoryStore::new(config.retention.ttl_seconds));

    // Build server with all tools
    let server = Server::builder()
        .name(&config.server.name)
        .register(SequentialThinkingTool::new(store.clone(), config.clone()))
        .register(SequentialThinkingAliasTool::new(store.clone(), config.clone()))
        .register(GetThinkingSessionTool::new(store.clone()))
        .register(ListThinkingSessionsTool::new(store.clone()))
        .register(ClearThinkingSessionTool::new(store.clone()))
        .register(ExportThinkingSessionTool::new(store.clone(), config.clone()))
        .build()?;

    // Serve over stdio
    server.serve_stdio().await?;
    Ok(())
}
```

### Tool implementations using rmcp

Each tool is a struct implementing the rmcp `Tool` trait or using the `#[tool]` macro:

```rust
#[derive(Clone)]
pub struct SequentialThinkingTool {
    store: Arc<MemoryStore>,
    config: AppConfig,
}

#[tool]
impl SequentialThinkingTool {
    #[tool(description = "Record one structured thought in a reasoning session.")]
    async fn sequentialthinking(
        &self,
        #[tool(param)] input: ThoughtInput,
    ) -> Result<SequentialThinkingOutput, ThinkingError> {
        handle_sequentialthinking(input, &*self.store, &self.config)
    }
}
```

### Tool annotations per spec section 10

| Tool | readOnlyHint | destructiveHint | idempotentHint | openWorldHint |
|---|---|---|---|---|
| sequentialthinking | false | false | false | false |
| sequential_thinking | false | false | false | false |
| get_thinking_session | true | false | true | false |
| list_thinking_sessions | true | false | true | false |
| export_thinking_session | true | false | true | false |
| clear_thinking_session | false | true | true | false |

### main.rs — entry point

```rust
use clap::Parser;

#[derive(Parser)]
#[command(name = "mcp-sequential-thinking")]
struct Cli {
    #[arg(long)]
    config: Option<PathBuf>,

    #[arg(long)]
    max_thoughts: Option<u32>,

    #[arg(long)]
    ttl_seconds: Option<u64>,

    #[arg(long)]
    disable_logging: bool,

    #[arg(long)]
    disable_redaction: bool,

    #[arg(long)]
    compatibility_mode: bool,

    #[arg(long, default_value = "info")]
    log_level: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let config = load_config(cli)?;
    run_server(config).await
}
```

### Config loading precedence

1. Default values (hardcoded in AppConfig::default())
2. Config file (if `--config` or `SEQUENTIAL_THINKING_CONFIG` provided)
3. Environment variables (`SEQUENTIAL_THINKING_TTL_SECONDS`, `SEQUENTIAL_THINKING_MAX_THOUGHTS`, etc.)
4. CLI flags (`--max-thoughts`, `--ttl-seconds`, etc.)

## File Changes

### New Files

| File | Purpose |
|---|---|
| `src/server.rs` | MCP server setup, tool registration, rmcp wiring |

### Modified Files

| File | Change |
|---|---|
| `src/main.rs` | Replace stub with full CLI + server bootstrap |

## Implementation Steps

1. Use Context7 MCP to fetch rmcp documentation for stdio server and tool registration
2. Implement `src/server.rs` — server builder, tool registration, stdio serve
3. Create rmcp tool structs for each of the 6 tools
4. Wire tool annotations (readOnlyHint, destructiveHint, etc.)
5. Implement `src/main.rs` — clap CLI, config loading, tracing init, server start
6. Handle graceful shutdown on SIGTERM/SIGINT
7. Build and test the binary manually
8. Write integration tests

## Data / API / Interface Contract

- Server name: `"mcp-sequential-thinking"` (from config)
- Transport: stdio (stdin/stdout)
- Server expects newline-delimited JSON-RPC messages per MCP spec
- Each tool's JSON schema is derived from its input type via schemars
- Errors are returned as MCP-compatible JSON-RPC error responses

## Error Handling

- Config file not found → error message to stderr, exit 1
- Invalid config JSON → error message to stderr, exit 1
- rmcp server error → log error, exit 1
- Tool handler errors → returned to client as MCP error response (server continues)
- SIGTERM/SIGINT → graceful shutdown, drain pending requests

## Observability

- Logs: `tracing::info!("mcp-sequential-thinking starting")` on startup; use `RUST_LOG` env var
- Metrics: None
- Traces: All tool calls logged at info level (metadata only)
- Alerts: None

## Testing Requirements

### Integration Tests

- Binary starts and accepts connections
- `initialize` MCP method returns server capabilities
- `tools/list` returns all 6 tools with correct schemas
- `tools/call` with sequentialthinking returns expected output
- Invalid JSON returns MCP-compatible error
- Server handles multiple sessions concurrently

## Validation Commands

```bash
cargo build --release                       # Release build
./target/release/mcp-sequential-thinking --help   # CLI help
cargo test --test mcp_contract              # MCP contract tests
```

## Acceptance Criteria

- [ ] `mcp-sequential-thinking` binary starts without errors
- [ ] All 6 tools registered and discoverable via `tools/list`
- [ ] `sequentialthinking` tool call returns correct output
- [ ] `sequential_thinking` alias returns identical output
- [ ] Tool annotations match spec section 10
- [ ] Config loads from file, env vars, and CLI flags with correct precedence
- [ ] Server handles SIGTERM gracefully
- [ ] Binary compiles with `cargo build --release`

## Risks

| Risk | Severity | Mitigation |
|---|---|---|
| rmcp API doesn't match our tool model | High | Use Context7 to verify before implementing; wrap in adapter if needed |
| rmcp schema generation differs from hand-written schemas | Medium | Verify with `tools/list` against spec examples |
| Tokio runtime conflicts with rmcp's async model | Low | Use `#[tokio::main]` as recommended by rmcp docs |
| Binary size too large | Low | Use `--release` with LTO; this is a local tool, not a deployed service |
