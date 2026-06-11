pub mod defaults;
pub mod rules;

use crate::config::AppConfig;
use crate::error::ThinkingError;
use crate::model::thought::ThoughtKind;
use crate::store::ThinkingStore;

use rules::ThoughtInput;

pub struct ValidationOutput {
    pub thought: crate::model::thought::ThoughtRecord,
    pub warnings: Vec<crate::model::warning::Warning>,
    pub redactions: Vec<crate::model::redaction::RedactionSummary>,
}

pub fn validate_thought(
    input: ThoughtInput,
    store: &dyn ThinkingStore,
    config: &AppConfig,
) -> Result<ValidationOutput, ThinkingError> {
    let mut warnings = Vec::new();
    let mut redactions = Vec::new();

    // Step 1: Retention cleanup is done by the store before validation

    // Step 2: Normalize legacy fields
    let input = rules::normalize_legacy(input, config, &mut warnings);

    // Step 3: Validate required fields
    rules::validate_required(&input)?;

    // Step 4: Apply defaults
    let kind = rules::validate_kind(&input)?;
    let status = rules::validate_status(&input, kind)?;

    // Step 5: Validate enum values
    rules::validate_branch_status(&input)?;

    // Step 6: Validate confidence range
    rules::validate_confidence(&input)?;

    // Step 7: Validate content length
    rules::validate_content_length(&input, config)?;

    // Step 8: Redact content
    let (content, reason_summary, thought_redactions) = rules::redact_content(&input, config);
    redactions.extend(thought_redactions);

    // Step 9: Validate branch references
    rules::validate_branch_refs(&input, store)?;

    // Step 10: Validate revision references
    let (is_revision, revises_thought) =
        rules::validate_revision(&input, kind, store, &mut warnings)?;

    // Step 11: Validate dependency references
    let depends_on = rules::validate_dependencies(&input, store)?;

    // Step 12: Enforce budgets
    rules::enforce_budgets(&input, store, config)?;

    // Build the thought record
    let session_id = input.session_id.clone().unwrap_or_default();
    let thought_number = input.thought_number.unwrap_or(1);
    let thought_id = format!("th_{:06}", thought_number);

    let thought = crate::model::thought::ThoughtRecord {
        thought_id,
        session_id: session_id.clone(),
        kind,
        content,
        thought_number,
        total_thoughts: input.total_thoughts.unwrap_or(1),
        next_thought_needed: input.next_thought_needed.unwrap_or(true),
        status,
        confidence: input.confidence,
        reason_summary,
        branch_id: input.branch_id.clone(),
        branch_label: input.branch_label.clone(),
        branch_from_thought: input.branch_from_thought,
        branch_status: input.branch_status,
        is_revision,
        revises_thought,
        depends_on,
        evidence: input.evidence.unwrap_or_default(),
        risk: input.risk.clone(),
        action_proposal: input.action_proposal.clone(),
        tags: input.tags.unwrap_or_default(),
        redactions: redactions.clone(),
        created_at: chrono::Utc::now(),
    };

    // Check terminal thought warning
    if !thought.next_thought_needed && thought.kind != ThoughtKind::FinalDecision {
        warnings.push(crate::model::warning::Warning {
            code: "non_final_terminal_thought".to_string(),
            message: "nextThoughtNeeded=false is usually expected with kind=final_decision."
                .to_string(),
        });
    }

    Ok(ValidationOutput {
        thought,
        warnings,
        redactions,
    })
}
