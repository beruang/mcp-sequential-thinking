use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum BranchStatus {
    Active,
    Selected,
    Rejected,
    Merged,
    Superseded,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct BranchSummary {
    pub branch_id: String,
    pub branch_label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_from_thought: Option<u32>,
    pub branch_status: BranchStatus,
    pub thought_count: usize,
}
