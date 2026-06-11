use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::error::ThinkingError;
use crate::store::ThinkingStore;

use super::ListSessionsOutput;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ListSessionsInput {
    #[serde(default = "default_limit")]
    pub limit: usize,
}

fn default_limit() -> usize {
    50
}

pub fn handle_list_sessions(
    input: ListSessionsInput,
    store: &dyn ThinkingStore,
) -> Result<ListSessionsOutput, ThinkingError> {
    let limit = input.limit.clamp(1, 500);
    let sessions = store.list_sessions(limit)?;
    Ok(ListSessionsOutput { sessions })
}
