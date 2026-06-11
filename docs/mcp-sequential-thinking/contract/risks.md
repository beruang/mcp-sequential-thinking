# Risks

## Risk register

| Risk | Severity | Likelihood | Mitigation | Owner |
|---|---|---|---|---|
| rmcp API is incompatible with our data model (schema derivation, tool annotations) | High | Possible | Use Context7 to verify rmcp API before implementation; wrap rmcp types if needed | Implementer |
| Redaction regexes produce false positives (redact non-secret content) | Medium | Possible | Use conservative patterns with word boundaries; log redaction counts but not content | Implementer |
| Budget exhaustion during critical reasoning | Medium | Possible | Allow `final_decision` after budget exceeded; make budgets configurable | Implementer |
| `Arc<RwLock<HashMap<...>>>` contention under concurrent access | Low | Unlikely | stdio is single-client; only concurrent if MCP client sends pipelined requests | Implementer |
| Legacy compatibility mode misinterprets modern fields | Medium | Unlikely | Only activate legacy normalization when `thought` field is present and `kind` is absent | Implementer |
| schemars JSON Schema generation differs from hand-written tool schemas | Low | Possible | Verify generated schemas against spec examples; use `#[schemars(...)]` attributes if needed | Implementer |
| TTL expiration race condition (session expires between check and use) | Low | Rare | Check expiration at the start of every tool call; accept small race window | Implementer |

## Watch list

- rmcp crate updates — monitor for breaking API changes after pinning
- Redaction pattern completeness — new secret formats may need new patterns
- Memory usage under high session count — TTL cleanup should keep this bounded

## Kill switches

- If rmcp proves incompatible with the data model, fall back to raw JSON-RPC over stdin/stdout using `serde_json` directly
- If `Arc<RwLock<...>>` contention is observed, migrate to `dashmap` (already in optional dependencies)
- If redaction causes data loss (false positives on critical content), disable via `--disable-redaction` flag
