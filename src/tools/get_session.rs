use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::error::ThinkingError;
use crate::store::ThinkingStore;

use super::GetSessionOutput;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetSessionInput {
    pub session_id: String,
    #[serde(default = "default_include_thoughts")]
    pub include_thoughts: bool,
}

fn default_include_thoughts() -> bool {
    true
}

pub fn handle_get_session(
    input: GetSessionInput,
    store: &dyn ThinkingStore,
) -> Result<GetSessionOutput, ThinkingError> {
    let session = store
        .get_session(&input.session_id)?
        .ok_or_else(|| ThinkingError::SessionNotFound(input.session_id.clone()))?;

    Ok(GetSessionOutput {
        session_id: session.session_id,
        status: format!("{:?}", session.status).to_lowercase(),
        created_at: session.created_at.to_rfc3339(),
        updated_at: session.updated_at.to_rfc3339(),
        thought_count: session.thoughts.len(),
        branch_count: session.branches.len(),
        branches: session.branches,
        thoughts: if input.include_thoughts {
            Some(session.thoughts)
        } else {
            None
        },
    })
}
