# Assumptions

| ! | Assumption | Confidence impact | What would invalidate |
|---|---|---|---|
| ! | rmcp crate provides a stable stdio transport API suitable for MCP tool registration | High — drives server.rs and all tool wiring | rmcp API is incompatible with our tool model; would need alternate MCP SDK |
| ! | tokio runtime is compatible with rmcp's async model | Medium — drives async runtime choice | rmcp requires a different runtime; would need sync wrapper or alternate SDK |
| | The target platform is macOS/Linux with Rust toolchain 1.80+ | Low — only affects build instructions | Target is Windows-only; would need platform-specific adjustments |
| | AI agents will use `sequentialthinking` for complex multi-step tasks, not simple queries | Low — affects usage documentation only | Agents use it for all tasks; would need performance optimization for high throughput |
| | Legacy clients send `thought`, `thoughtNumber`, `totalThoughts`, `nextThoughtNeeded` | Medium — drives compatibility mode design | Legacy clients use different field names; would need to extend normalization |
| | Session IDs are provided by the client (not auto-generated for structured input) | Medium — drives session creation logic | Clients expect server-generated session IDs; would need auto-generation |
| | In-memory storage is acceptable for production use (no persistence needed) | Low — drives store design | Users demand persistence; would need a storage backend |

## Open questions

None — the spec resolves all design questions explicitly.

## Unknowns

- Exact rmcp API surface for registering tools with annotations (will resolve via Context7 before implementation)
- Exact schemars derive macro behavior for nested enum serialization (will resolve during implementation)
- Performance characteristics of `Arc<RwLock<HashMap<...>>>` under concurrent MCP client load (acceptable for initial implementation)
