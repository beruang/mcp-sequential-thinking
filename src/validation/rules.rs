use std::collections::HashSet;

use crate::config::AppConfig;
use crate::error::ThinkingError;
use crate::model::action::ActionProposal;
use crate::model::branch::BranchStatus;
use crate::model::evidence::EvidenceRef;
use crate::model::redaction::RedactionSummary;
use crate::model::risk::RiskInfo;
use crate::model::thought::{ThoughtKind, ThoughtStatus};
use crate::model::warning::Warning;
use crate::redaction::redact_text;
use crate::store::ThinkingStore;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ... keep existing imports

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ThoughtInput {
    pub session_id: Option<String>,
    pub kind: Option<String>,
    pub content: Option<String>,
    pub thought: Option<String>,
    pub thought_number: Option<u32>,
    pub total_thoughts: Option<u32>,
    pub next_thought_needed: Option<bool>,
    pub status: Option<String>,
    pub confidence: Option<f32>,
    pub reason_summary: Option<String>,
    pub branch_id: Option<String>,
    pub branch_label: Option<String>,
    pub branch_from_thought: Option<u32>,
    pub branch_status: Option<BranchStatus>,
    pub is_revision: Option<bool>,
    pub revises_thought: Option<u32>,
    pub depends_on: Option<Vec<u32>>,
    pub evidence: Option<Vec<EvidenceRef>>,
    pub risk: Option<RiskInfo>,
    pub action_proposal: Option<ActionProposal>,
    pub tags: Option<Vec<String>>,
}

pub fn normalize_legacy(
    mut input: ThoughtInput,
    config: &AppConfig,
    warnings: &mut Vec<Warning>,
) -> ThoughtInput {
    if config.compatibility.accept_legacy_thought_field
        && input.content.is_none()
        && input.thought.is_some()
    {
        input.content = input.thought.take();
        if input.kind.is_none() {
            input.kind = Some(config.compatibility.default_legacy_kind.clone());
        }
        if input.session_id.is_none() {
            input.session_id = Some(config.compatibility.default_session_id.clone());
        }
        warnings.push(Warning {
            code: "legacy_input".to_string(),
            message: "Field 'thought' was mapped to 'content'. Consider using structured input."
                .to_string(),
        });
    }
    input
}

pub fn validate_required(input: &ThoughtInput) -> Result<(), ThinkingError> {
    // Legacy mode: session ID may be defaulted
    if input.session_id.is_none() || input.session_id.as_deref() == Some("") {
        return Err(ThinkingError::InvalidInput {
            message: "sessionId is required".to_string(),
            field: Some("sessionId".to_string()),
        });
    }
    if input.kind.is_none() {
        return Err(ThinkingError::InvalidInput {
            message: "kind is required".to_string(),
            field: Some("kind".to_string()),
        });
    }
    if input
        .content
        .as_deref()
        .map(str::trim)
        .unwrap_or("")
        .is_empty()
    {
        return Err(ThinkingError::InvalidInput {
            message: "content is required".to_string(),
            field: Some("content".to_string()),
        });
    }
    if input.thought_number.unwrap_or(0) < 1 {
        return Err(ThinkingError::InvalidInput {
            message: "thoughtNumber must be >= 1".to_string(),
            field: Some("thoughtNumber".to_string()),
        });
    }
    if input.total_thoughts.unwrap_or(0) < 1 {
        return Err(ThinkingError::InvalidInput {
            message: "totalThoughts must be >= 1".to_string(),
            field: Some("totalThoughts".to_string()),
        });
    }
    Ok(())
}

pub fn validate_kind(input: &ThoughtInput) -> Result<ThoughtKind, ThinkingError> {
    let kind_str = input.kind.as_deref().unwrap_or("");
    serde_json::from_str(&format!("\"{}\"", kind_str))
        .map_err(|_| ThinkingError::InvalidKind(kind_str.to_string()))
}

pub fn validate_status(
    input: &ThoughtInput,
    kind: ThoughtKind,
) -> Result<ThoughtStatus, ThinkingError> {
    if let Some(ref status_str) = input.status {
        serde_json::from_str(&format!("\"{}\"", status_str))
            .map_err(|_| ThinkingError::InvalidStatus(status_str.to_string()))
    } else {
        Ok(super::defaults::default_status_for_kind(kind))
    }
}

pub fn validate_branch_status(input: &ThoughtInput) -> Result<(), ThinkingError> {
    // branch_status comes as an enum already parsed; no validation needed here
    let _ = input;
    Ok(())
}

pub fn validate_confidence(input: &ThoughtInput) -> Result<(), ThinkingError> {
    if let Some(conf) = input.confidence {
        if !(0.0..=1.0).contains(&conf) {
            return Err(ThinkingError::InvalidConfidence);
        }
    }
    Ok(())
}

pub fn validate_content_length(
    input: &ThoughtInput,
    config: &AppConfig,
) -> Result<(), ThinkingError> {
    if let Some(ref content) = input.content {
        if content.len() > config.budgets.max_content_chars {
            return Err(ThinkingError::ContentTooLarge(
                "content exceeds maxContentChars".to_string(),
            ));
        }
    }
    if let Some(ref summary) = input.reason_summary {
        if summary.len() > config.budgets.max_reason_summary_chars {
            return Err(ThinkingError::ContentTooLarge(
                "reasonSummary exceeds maxReasonSummaryChars".to_string(),
            ));
        }
    }
    Ok(())
}

pub fn redact_content(
    input: &ThoughtInput,
    config: &AppConfig,
) -> (String, Option<String>, Vec<RedactionSummary>) {
    if !config.redaction.enabled || !config.redaction.redact_before_store {
        return (
            input.content.clone().unwrap_or_default(),
            input.reason_summary.clone(),
            vec![],
        );
    }

    let mut all_redactions = Vec::new();
    let content = input.content.clone().unwrap_or_default();
    let result = redact_text(&content);
    all_redactions.extend(result.redactions);

    let reason_summary = if let Some(ref summary) = input.reason_summary {
        let r = redact_text(summary);
        all_redactions.extend(r.redactions);
        Some(r.content)
    } else {
        None
    };

    (result.content, reason_summary, all_redactions)
}

pub fn validate_branch_refs(
    input: &ThoughtInput,
    store: &dyn ThinkingStore,
) -> Result<(), ThinkingError> {
    if input.branch_from_thought.is_some() && input.branch_id.is_none() {
        return Err(ThinkingError::InvalidBranch(
            "branchId is required when branchFromThought is set".to_string(),
        ));
    }
    if let Some(ref_num) = input.branch_from_thought {
        if let Some(ref sid) = input.session_id {
            match store.get_session(sid)? {
                Some(session) => {
                    if !session.thoughts.iter().any(|t| t.thought_number == ref_num) {
                        return Err(ThinkingError::InvalidBranch(format!(
                            "branchFromThought references non-existent thought {}",
                            ref_num
                        )));
                    }
                }
                None => {
                    // New session — no thoughts exist, so any reference is invalid
                    return Err(ThinkingError::InvalidBranch(format!(
                        "branchFromThought references non-existent thought {} (session is new)",
                        ref_num
                    )));
                }
            }
        }
    }
    Ok(())
}

pub fn validate_revision(
    input: &ThoughtInput,
    kind: ThoughtKind,
    store: &dyn ThinkingStore,
    warnings: &mut Vec<Warning>,
) -> Result<(bool, Option<u32>), ThinkingError> {
    let is_revision = if kind == ThoughtKind::Revision {
        true
    } else if input.is_revision.unwrap_or(false) {
        warnings.push(Warning {
            code: "kind_normalized_to_revision".to_string(),
            message: "isRevision=true implies kind=revision; normalized kind to revision."
                .to_string(),
        });
        true
    } else {
        input.is_revision.unwrap_or(false)
    };

    let revises_thought = input.revises_thought;

    if is_revision && revises_thought.is_none() {
        return Err(ThinkingError::InvalidRevision(
            "revisesThought is required when kind=revision".to_string(),
        ));
    }

    if let Some(ref_num) = revises_thought {
        if let Some(ref sid) = input.session_id {
            match store.get_session(sid)? {
                Some(session) => {
                    if !session.thoughts.iter().any(|t| t.thought_number == ref_num) {
                        return Err(ThinkingError::InvalidRevision(format!(
                            "revisesThought references non-existent thought {}",
                            ref_num
                        )));
                    }
                }
                None => {
                    return Err(ThinkingError::InvalidRevision(format!(
                        "revisesThought references non-existent thought {} (session is new)",
                        ref_num
                    )));
                }
            }
        }
    }

    Ok((is_revision, revises_thought))
}

pub fn validate_dependencies(
    input: &ThoughtInput,
    store: &dyn ThinkingStore,
) -> Result<Vec<u32>, ThinkingError> {
    let mut deps = input.depends_on.clone().unwrap_or_default();

    // Deduplicate
    let mut seen = HashSet::new();
    deps.retain(|d| seen.insert(*d));

    let current_num = input.thought_number.unwrap_or(0);

    for dep in &deps {
        if *dep == current_num {
            return Err(ThinkingError::InvalidDependency(
                "A thought cannot depend on itself".to_string(),
            ));
        }
        if *dep > current_num {
            return Err(ThinkingError::InvalidDependency(
                "A thought cannot depend on a future thought".to_string(),
            ));
        }
    }

    // Check that all dependencies exist
    if !deps.is_empty() {
        if let Some(ref sid) = input.session_id {
            if let Some(session) = store.get_session(sid)? {
                for dep in &deps {
                    if !session.thoughts.iter().any(|t| t.thought_number == *dep) {
                        return Err(ThinkingError::InvalidDependency(format!(
                            "Dependency on non-existent thought {}",
                            dep
                        )));
                    }
                }
            }
        }
    }

    Ok(deps)
}

pub fn enforce_budgets(
    input: &ThoughtInput,
    store: &dyn ThinkingStore,
    config: &AppConfig,
) -> Result<(), ThinkingError> {
    let sid = input.session_id.as_deref().unwrap_or("");
    let session = store.get_session(sid)?;

    let thought_count = session.as_ref().map(|s| s.thoughts.len()).unwrap_or(0) as u32;
    let budget = &config.budgets;
    let kind = input
        .kind
        .as_deref()
        .and_then(|k| serde_json::from_str::<ThoughtKind>(&format!("\"{}\"", k)).ok());

    // Allow final_decision after budget exceeded
    let is_final = kind == Some(ThoughtKind::FinalDecision);

    if thought_count >= budget.max_thoughts_per_session
        && !(is_final && config.behavior.allow_final_decision_after_budget_exceeded)
    {
        return Err(ThinkingError::BudgetExceeded(
            "Maximum thoughts per session exceeded".to_string(),
        ));
    }

    // Count branches
    if let Some(ref sess) = session {
        let revision_count = sess.thoughts.iter().filter(|t| t.is_revision).count() as u32;

        if (input.is_revision.unwrap_or(false) || kind == Some(ThoughtKind::Revision))
            && revision_count >= budget.max_revisions_per_session
        {
            return Err(ThinkingError::BudgetExceeded(
                "Maximum revisions per session exceeded".to_string(),
            ));
        }

        // New branch check
        if let Some(ref bid) = input.branch_id {
            let existing_branch = sess.branches.iter().any(|b| b.branch_id == *bid);
            if !existing_branch && sess.branches.len() as u32 >= budget.max_branches_per_session {
                return Err(ThinkingError::BudgetExceeded(
                    "Maximum branches per session exceeded".to_string(),
                ));
            }
        }
    }

    // Evidence count
    if let Some(ref evidence) = input.evidence {
        if evidence.len() > budget.max_evidence_items_per_thought {
            return Err(ThinkingError::BudgetExceeded(
                "Maximum evidence items per thought exceeded".to_string(),
            ));
        }
    }

    // Tags count
    if let Some(ref tags) = input.tags {
        if tags.len() > budget.max_tags_per_thought {
            return Err(ThinkingError::BudgetExceeded(
                "Maximum tags per thought exceeded".to_string(),
            ));
        }
    }

    Ok(())
}
