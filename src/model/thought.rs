use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::action::ActionProposal;
use super::branch::BranchStatus;
use super::evidence::EvidenceRef;
use super::redaction::RedactionSummary;
use super::risk::RiskInfo;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ThoughtKind {
    Observation,
    Assumption,
    Constraint,
    Risk,
    Option,
    Decision,
    Revision,
    Validation,
    NextAction,
    Blocker,
    Question,
    FinalDecision,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ThoughtStatus {
    Unverified,
    Verified,
    Rejected,
    Superseded,
    Active,
    Done,
    Blocked,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ThoughtRecord {
    pub thought_id: String,
    pub session_id: String,

    pub kind: ThoughtKind,
    pub content: String,

    pub thought_number: u32,
    pub total_thoughts: u32,
    pub next_thought_needed: bool,

    pub status: ThoughtStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_summary: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_from_thought: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_status: Option<BranchStatus>,

    pub is_revision: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revises_thought: Option<u32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub depends_on: Vec<u32>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence: Vec<EvidenceRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk: Option<RiskInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_proposal: Option<ActionProposal>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub redactions: Vec<RedactionSummary>,

    pub created_at: DateTime<Utc>,
}
