use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RiskLevel {
    None,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RiskCategory {
    Privacy,
    Security,
    DestructiveAction,
    ExternalSideEffect,
    DataLoss,
    CredentialExposure,
    Cost,
    Legal,
    Operational,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RiskInfo {
    pub level: RiskLevel,
    pub category: RiskCategory,
    #[serde(default)]
    pub requires_confirmation: bool,
    pub description: String,
}
