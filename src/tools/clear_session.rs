use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::error::ThinkingError;
use crate::store::ThinkingStore;

use super::ClearSessionOutput;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClearSessionInput {
    pub session_id: String,
}

pub fn handle_clear_session(
    input: ClearSessionInput,
    store: &dyn ThinkingStore,
) -> Result<ClearSessionOutput, ThinkingError> {
    let cleared = store.clear_session(&input.session_id)?;
    Ok(ClearSessionOutput {
        session_id: input.session_id,
        cleared,
    })
}
