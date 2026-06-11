use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct BudgetState {
    pub max_thoughts: u32,
    pub thoughts_used: u32,
    pub thoughts_remaining: u32,
    pub max_branches: u32,
    pub branches_used: u32,
    pub branches_remaining: u32,
    pub max_revisions: u32,
    pub revisions_used: u32,
    pub revisions_remaining: u32,
}
