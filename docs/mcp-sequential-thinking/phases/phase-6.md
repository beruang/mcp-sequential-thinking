# Phase 6: MCP Server Integration

**Status:** Draft
**Depends on:** phase-4, phase-5
**Risk:** Medium
**Value:** Wires everything together into a working MCP server over stdio. This is where the binary becomes functional.

## Goal

Wire all tool handlers into the rmcp MCP server. Register all 6 tools with correct schemas and annotations. Implement `main.rs` with CLI flag parsing, config loading, and server bootstrap.

## Produces

- `src/server.rs`
- `src/main.rs` (final version)

## Verification

```bash
cargo build --release           # Release binary compiles
./target/release/mcp-sequential-thinking --help   # CLI help works
```
