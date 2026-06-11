use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::risk::RiskInfo;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ActionProposal {
    pub tool: String,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub args: HashMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk: Option<RiskInfo>,
    #[serde(default)]
    pub requires_approval: bool,
}
