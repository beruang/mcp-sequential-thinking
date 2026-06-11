use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::branch::BranchSummary;
use super::thought::ThoughtRecord;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SessionStatus {
    Active,
    Completed,
    Expired,
    Cleared,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ThinkingSession {
    pub session_id: String,
    pub status: SessionStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub thoughts: Vec<ThoughtRecord>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub branches: Vec<BranchSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct SessionSummary {
    pub session_id: String,
    pub status: SessionStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub thought_count: usize,
    pub branch_count: usize,
}

impl ThinkingSession {
    pub fn new(session_id: String) -> Self {
        let now = Utc::now();
        Self {
            session_id,
            status: SessionStatus::Active,
            created_at: now,
            updated_at: now,
            thoughts: Vec::new(),
            branches: Vec::new(),
        }
    }

    pub fn to_summary(&self) -> SessionSummary {
        SessionSummary {
            session_id: self.session_id.clone(),
            status: self.status,
            created_at: self.created_at,
            updated_at: self.updated_at,
            thought_count: self.thoughts.len(),
            branch_count: self.branches.len(),
        }
    }
}
