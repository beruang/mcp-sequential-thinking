use crate::config::AppConfig;
use crate::error::ThinkingError;
use crate::model::budget::BudgetState;
use crate::store::ThinkingStore;
use crate::validation::{self, rules::ThoughtInput};

use super::SequentialThinkingOutput;

pub fn handle_sequentialthinking(
    input: ThoughtInput,
    store: &dyn ThinkingStore,
    config: &AppConfig,
) -> Result<SequentialThinkingOutput, ThinkingError> {
    let thought_num = input.thought_number.unwrap_or(1);
    let total = input.total_thoughts.unwrap_or(1);
    let next_needed = input.next_thought_needed.unwrap_or(true);

    let output = validation::validate_thought(input, store, config)?;
    let thought = output.thought;
    let sid = thought.session_id.clone();

    // Insert into store
    store.upsert_thought(thought.clone())?;

    // Get session for metadata
    let session = store.get_session(&sid)?;
    let thought_history_length = session.as_ref().map(|s| s.thoughts.len()).unwrap_or(0);
    let session_status = session
        .as_ref()
        .map(|s| format!("{:?}", s.status).to_lowercase())
        .unwrap_or_else(|| "active".to_string());

    // Count thoughts and revisions for budget
    let thoughts_used = session
        .as_ref()
        .map(|s| s.thoughts.len() as u32)
        .unwrap_or(0);
    let revision_count = session
        .as_ref()
        .map(|s| s.thoughts.iter().filter(|t| t.is_revision).count() as u32)
        .unwrap_or(0);
    let branch_count = session
        .as_ref()
        .map(|s| s.branches.len() as u32)
        .unwrap_or(0);

    let budget = BudgetState {
        max_thoughts: config.budgets.max_thoughts_per_session,
        thoughts_used,
        thoughts_remaining: config
            .budgets
            .max_thoughts_per_session
            .saturating_sub(thoughts_used),
        max_branches: config.budgets.max_branches_per_session,
        branches_used: branch_count,
        branches_remaining: config
            .budgets
            .max_branches_per_session
            .saturating_sub(branch_count),
        max_revisions: config.budgets.max_revisions_per_session,
        revisions_used: revision_count,
        revisions_remaining: config
            .budgets
            .max_revisions_per_session
            .saturating_sub(revision_count),
    };

    let branches = session
        .as_ref()
        .map(|s| s.branches.clone())
        .unwrap_or_default();

    Ok(SequentialThinkingOutput {
        session_id: sid,
        thought_id: thought.thought_id,
        thought_number: thought_num,
        total_thoughts: total,
        next_thought_needed: next_needed,
        accepted: true,
        session_status,
        branches,
        thought_history_length,
        budget,
        redactions: output.redactions,
        warnings: output.warnings,
    })
}

pub fn handle_sequential_thinking(
    input: ThoughtInput,
    store: &dyn ThinkingStore,
    config: &AppConfig,
) -> Result<SequentialThinkingOutput, ThinkingError> {
    handle_sequentialthinking(input, store, config)
}
