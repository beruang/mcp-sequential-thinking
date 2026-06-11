# Success Criteria

## Acceptance criteria

- [ ] `mcp-sequential-thinking` binary starts and registers all 6 MCP tools over stdio
- [ ] `sequentialthinking` tool accepts structured input and returns thought with budget, branches, warnings, redactions
- [ ] `sequential_thinking` alias produces identical output to `sequentialthinking` for the same input
- [ ] `get_thinking_session` returns session metadata and thoughts when `includeThoughts=true`
- [ ] `list_thinking_sessions` returns active sessions up to configured limit
- [ ] `clear_thinking_session` removes a session and is idempotent (returns cleared:false for missing sessions)
- [ ] `export_thinking_session` returns JSON, JSONL, and Markdown exports
- [ ] Legacy input (`thought` field) is normalized to structured format with `legacy_input` warning
- [ ] Budget exceeded returns `budget_exceeded` error for thoughts, branches, revisions, content, evidence, tags
- [ ] Confidence outside [0.0, 1.0] returns `invalid_confidence` error
- [ ] Invalid kind returns `invalid_kind` error
- [ ] Missing sessionId in structured mode returns `missing_session_id` error
- [ ] Revision to non-existent thought returns `invalid_revision` error
- [ ] Branch from non-existent thought returns `invalid_branch` error
- [ ] Dependency on future or self returns `invalid_dependency` error
- [ ] Secrets are redacted before storage (Bearer tokens, API keys, passwords, etc.)
- [ ] Raw thought content is never logged (only metadata: session, thoughtNumber, kind, status)
- [ ] Sessions expire after TTL and are removed by opportunistic cleanup
- [ ] Configuration loads from file, env vars, and CLI flags with correct precedence

## Validation checks

```bash
cargo build --release                    # Binary compiles
cargo test                                # All tests pass
cargo clippy -- -D warnings              # No clippy warnings
cargo fmt -- --check                      # Proper formatting
```

## Artifact checks

- [ ] `Cargo.toml` has no wildcard version dependencies
- [ ] `README.md` includes: purpose, privacy warning, installation, client config, tool list, schema examples, legacy mode, config, env vars, CLI flags, security model, non-goals, test instructions
- [ ] `LICENSE` file exists
- [ ] `.gitignore` covers `target/` and sensitive files
- [ ] `src/main.rs` is the binary entrypoint
- [ ] All source files from the repository layout (Section 28 of spec) exist

## Quality checks

- [ ] All 12 thought kinds, 7 thought statuses, 5 branch statuses, 5 risk levels, 10 risk categories implemented as enums with serde rename_all = "snake_case"
- [ ] All 15 error codes implemented as structured MCP-compatible errors
- [ ] All 7 budget types enforced with correct default values
- [ ] Redaction covers: Bearer tokens, Authorization headers, API keys, private keys, password assignments, database URLs with credentials, GitHub tokens, AWS access keys, OpenAI-style keys, JWTs
- [ ] 10 test modules exist and pass
- [ ] CLI flags: --config, --max-thoughts, --ttl-seconds, --disable-logging, --disable-redaction, --compatibility-mode, --log-level

## Done/not-done boundary

**Done:** Binary compiles, all tests pass, all 6 tools respond correctly, budgets enforced, redaction active, legacy mode works, export works, README complete, Cargo.toml pinned.

**Not done:** Any missing tool, test failure, unpinned dependency, missing README section, or non-functional export format.
