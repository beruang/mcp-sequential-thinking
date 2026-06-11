use serde::{Deserialize, Serialize};

use crate::model::branch::BranchSummary;
use crate::model::budget::BudgetState;
use crate::model::redaction::RedactionSummary;
use crate::model::warning::Warning;

pub mod clear_session;
pub mod export_session;
pub mod get_session;
pub mod list_sessions;
pub mod sequentialthinking;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SequentialThinkingOutput {
    pub session_id: String,
    pub thought_id: String,
    pub thought_number: u32,
    pub total_thoughts: u32,
    pub next_thought_needed: bool,
    pub accepted: bool,
    pub session_status: String,
    pub branches: Vec<BranchSummary>,
    pub thought_history_length: usize,
    pub budget: BudgetState,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub redactions: Vec<RedactionSummary>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<Warning>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSessionOutput {
    pub session_id: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
    pub thought_count: usize,
    pub branch_count: usize,
    pub branches: Vec<BranchSummary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thoughts: Option<Vec<crate::model::thought::ThoughtRecord>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListSessionsOutput {
    pub sessions: Vec<crate::model::session::SessionSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClearSessionOutput {
    pub session_id: String,
    pub cleared: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportSessionOutput {
    pub format: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}
