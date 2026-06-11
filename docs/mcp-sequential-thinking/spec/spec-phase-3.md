# Spec Phase 3: Validation & Redaction Pipeline

## Phase Goal

Implement the 17-step validation pipeline for every `sequentialthinking` call. Implement secret redaction covering 10+ patterns.

## Dependencies

- Requires: Phase 2 (store for reference validation, e.g., checking that branch/dependency/revision targets exist)
- Produces: `src/validation/mod.rs`, `src/validation/rules.rs`, `src/validation/defaults.rs`, `src/redaction/mod.rs`, `src/redaction/patterns.rs`

## Existing Code References

- Pattern to follow: Pipeline pattern — each step takes input, returns Result or modified input
- Related module: `src/model/thought.rs` (ThoughtKind, ThoughtStatus), `src/model/risk.rs` (RiskLevel), `src/error.rs` (ThinkingError, ErrorCode), `src/store/memory.rs` (for reference lookups)
- Test pattern: Each validation rule tested independently, then pipeline integration test

## Technical Approach

### Validation Pipeline (17 steps)

```rust
pub fn validate_thought(
    input: ThoughtInput,
    store: &dyn ThinkingStore,
    config: &AppConfig,
) -> Result<ValidationOutput, ThinkingError> {
    // 1. Run retention cleanup
    // 2. Normalize legacy fields
    // 3. Validate required fields
    // 4. Apply default values
    // 5. Validate enum values
    // 6. Validate confidence range
    // 7. Validate content length
    // 8. Redact content if configured
    // 9. Validate branch references
    // 10. Validate revision references
    // 11. Validate dependency references
    // 12. Enforce budgets
    // 13. (Insert thought — done by store)
    // 14. (Update branch summaries — done by store)
    // 15. (Auto-supersede revised thoughts — done by store)
    // 16. (Update session timestamps — done by store)
    // 17. Return response
}
```

### Step details

1. **Retention cleanup** — call `store.cleanup_expired()`
2. **Legacy normalization** — if `thought` field present and `content` absent: map `thought` → `content`, use `defaultLegacyKind` for kind, use `defaultSessionId` for sessionId, add `legacy_input` warning
3. **Required fields** — sessionId non-empty, kind present, content non-empty after trim, thoughtNumber >= 1, totalThoughts >= 1
4. **Default values** — apply status default based on kind (per spec section 13), branchId default "main", branchLabel default "main", branchStatus default "active", isRevision default based on kind
5. **Enum values** — kind is valid ThoughtKind variant, status is valid ThoughtStatus variant, branchStatus is valid BranchStatus variant, risk level/category are valid if present
6. **Confidence range** — if present, must be 0.0 <= confidence <= 1.0
7. **Content length** — content.len() <= maxContentChars, reasonSummary.len() <= maxReasonSummaryChars if present
8. **Redaction** — if enabled, scan content and reasonSummary for secrets, replace with `[REDACTED:<kind>]`, record RedactionSummary entries
9. **Branch references** — if branchFromThought present, branchId required; branchFromThought must reference existing thought in same session
10. **Revision references** — if kind=revision or isRevision=true: revisesThought required, must reference existing thought in same session; if isRevision=true but kind!=revision: normalize kind to revision, add `kind_normalized_to_revision` warning
11. **Dependency references** — each dependency must exist in session, cannot be self, cannot be future, deduplicate
12. **Budget enforcement** — check thought count < maxThoughtsPerSession (unless this is final_decision and allowFinalDecisionAfterBudgetExceeded), branch count < maxBranchesPerSession, revision count < maxRevisionsPerSession, evidence count <= maxEvidenceItemsPerThought, tags count <= maxTagsPerThought

### Redaction patterns

```rust
pub static REDACTION_PATTERNS: &[RedactionPattern] = &[
    // Bearer tokens
    RedactionPattern { kind: "bearer_token", regex: r"(?i)bearer\s+[a-zA-Z0-9\-._~+/]+=*" },
    // Authorization headers
    RedactionPattern { kind: "auth_header", regex: r"(?i)authorization:\s*[^\s]+" },
    // API keys (generic pattern)
    RedactionPattern { kind: "api_key", regex: r"(?i)(api[_-]?key|apikey)\s*[:=]\s*[^\s]+" },
    // Private keys (PEM)
    RedactionPattern { kind: "private_key", regex: r"-----BEGIN\s+(RSA\s+)?PRIVATE\s+KEY-----" },
    // Password assignments
    RedactionPattern { kind: "password", regex: r"(?i)password\s*[:=]\s*[^\s]+" },
    // DB URLs with credentials
    RedactionPattern { kind: "db_credentials", regex: r"(?i)(mysql|postgres|postgresql|mongodb)://[^:@]+:[^@]+@" },
    // GitHub tokens
    RedactionPattern { kind: "github_token", regex: r"(?i)gh[ps]_[a-zA-Z0-9]{36}" },
    // AWS access keys
    RedactionPattern { kind: "aws_key", regex: r"(?i)AKIA[0-9A-Z]{16}" },
    // AWS secret keys
    RedactionPattern { kind: "aws_secret", regex: r"(?i)aws(_|-)secret[\"']?\s*[:=]\s*[\"'][^\"']+[\"']" },
    // OpenAI-style keys
    RedactionPattern { kind: "openai_key", regex: r"(?i)sk-[a-zA-Z0-9]{32,}" },
    // JWTs
    RedactionPattern { kind: "jwt", regex: r"eyJ[a-zA-Z0-9\-_]+\.eyJ[a-zA-Z0-9\-_]+\.[a-zA-Z0-9\-_]+" },
];
```

### Defaults

```rust
pub fn default_status_for_kind(kind: ThoughtKind) -> ThoughtStatus {
    match kind {
        ThoughtKind::Observation => ThoughtStatus::Verified,
        ThoughtKind::Assumption => ThoughtStatus::Unverified,
        ThoughtKind::Constraint | ThoughtKind::Risk | ThoughtKind::Option
        | ThoughtKind::NextAction | ThoughtKind::Question => ThoughtStatus::Active,
        ThoughtKind::Decision | ThoughtKind::Revision | ThoughtKind::Validation
        | ThoughtKind::FinalDecision => ThoughtStatus::Done,
        ThoughtKind::Blocker => ThoughtStatus::Blocked,
    }
}
```

## File Changes

### New Files

| File | Purpose |
|---|---|
| `src/validation/mod.rs` | Pipeline orchestrator, ValidationOutput struct |
| `src/validation/rules.rs` | Individual validation functions (one per step) |
| `src/validation/defaults.rs` | Default value application logic |
| `src/redaction/mod.rs` | Redaction engine, RedactionPattern struct |
| `src/redaction/patterns.rs` | Regex patterns for 10+ secret types |

## Implementation Steps

1. Define `ValidationOutput` struct (thought: ThoughtRecord, warnings: Vec<Warning>, redactions: Vec<RedactionSummary>, budget: BudgetState, sessionStatus: SessionStatus, branches: Vec<BranchSummary>)
2. Implement each validation step as a separate function returning `Result<(), ThinkingError>` or `Result<ModifiedInput, ThinkingError>`
3. Implement legacy normalization (step 2)
4. Implement required field validation (step 3)
5. Implement default value application (step 4)
6. Implement enum value validation (step 5)
7. Implement confidence and content length validation (steps 6–7)
8. Implement redaction engine with regex patterns (step 8)
9. Implement branch, revision, dependency reference validation (steps 9–11)
10. Implement budget enforcement (step 12)
11. Wire all steps into pipeline orchestrator
12. Write unit tests for each step
13. Write integration test for full pipeline

## Data / API / Interface Contract

- `validate_thought(ThoughtInput, &dyn ThinkingStore, &AppConfig) -> Result<ValidationOutput, ThinkingError>`
- `ThoughtInput` is the raw JSON input from the MCP client (all optional fields)
- `ValidationOutput` contains the validated/redacted/applied ThoughtRecord plus metadata
- Steps that fail return `Err(ThinkingError)` with the appropriate error code
- Steps that succeed but have warnings add to `warnings` vec
- Redaction modifies content in-place and adds to `redactions` vec

## Error Handling

- Required field missing → `invalid_input` with field name
- Invalid kind → `invalid_kind`
- Invalid status → `invalid_status`
- Confidence out of range → `invalid_confidence`
- Content too large → `content_too_large`
- Invalid branch reference → `invalid_branch`
- Invalid revision reference → `invalid_revision`
- Invalid dependency → `invalid_dependency`
- Budget exceeded → `budget_exceeded`
- Redaction engine failure → `redaction_failed`

## Observability

- Logs: `tracing::debug!("validation pipeline: step={} session={} thought={}", step, session_id, thought_number)` (metadata only)
- Metrics: None
- Traces: Validation errors include field-level detail
- Alerts: None

## Testing Requirements

### Unit Tests

- Legacy normalization: `thought` field maps to `content` with correct defaults
- Legacy normalization: warning `legacy_input` added
- Required field: empty sessionId rejected
- Required field: empty content rejected
- Required field: thoughtNumber < 1 rejected
- Required field: totalThoughts < 1 rejected
- Default status: each kind maps to correct default
- Default branch: "main" when none provided
- Invalid kind rejected
- Invalid status rejected
- Confidence -0.1 rejected
- Confidence 1.2 rejected
- Confidence 0.75 accepted
- Confidence None accepted
- Content exceeding maxContentChars rejected
- Reason summary exceeding maxReasonSummaryChars rejected
- Evidence count exceeding maxEvidenceItemsPerThought rejected
- Tags count exceeding maxTagsPerThought rejected
- Branch reference to non-existent thought rejected
- Revision to non-existent thought rejected
- Dependency on self rejected
- Dependency on future thought rejected
- Missing dependency rejected
- Duplicate dependencies deduplicated
- Budget exceeded for thoughts rejected
- Budget exceeded but final_decision allowed (when configured)
- Auto-supersede marks revised thought as superseded

### Integration Tests

- Full pipeline: valid input produces valid output
- Full pipeline: invalid input produces correct error code and message
- Full pipeline: legacy input produces output with legacy warning

## Validation Commands

```bash
cargo test --lib validation        # Inner loop: validation tests
cargo test --lib redaction         # Inner loop: redaction tests
cargo build                         # Compile check
```

## Acceptance Criteria

- [ ] All 17 validation steps implemented in order
- [ ] Legacy normalization maps `thought` → `content` with defaults
- [ ] All required field validations reject on empty/invalid values
- [ ] All 12 default statuses correct per kind
- [ ] Confidence range [0.0, 1.0] enforced
- [ ] All 7 budget types enforced
- [ ] Branch, revision, dependency references validated
- [ ] 10+ redaction patterns defined and functional
- [ ] Redaction modifies content before store and before log
- [ ] All unit and integration tests pass

## Risks

| Risk | Severity | Mitigation |
|---|---|---|
| Regex patterns cause false positives on legitimate content | Medium | Use conservative patterns; log redaction counts but not content |
| Budget enforcement blocks valid reasoning | Medium | Allow final_decision after budget exceeded; make budgets configurable |
| Pipeline ordering is fragile | Low | Each step is a separate function; order change only affects orchestrator |
