use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::error::ThinkingError;
use crate::export;
use crate::store::ThinkingStore;

use super::ExportSessionOutput;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ExportSessionInput {
    pub session_id: String,
    #[serde(default = "default_format")]
    pub format: String,
}

fn default_format() -> String {
    "json".to_string()
}

pub fn handle_export_session(
    input: ExportSessionInput,
    store: &dyn ThinkingStore,
) -> Result<ExportSessionOutput, ThinkingError> {
    let session = store
        .get_session(&input.session_id)?
        .ok_or_else(|| ThinkingError::SessionNotFound(input.session_id.clone()))?;

    match input.format.as_str() {
        "json" => {
            let json_session = serde_json::to_value(&session)
                .map_err(|e| ThinkingError::SerializationError(e.to_string()))?;
            Ok(ExportSessionOutput {
                format: "json".to_string(),
                session: Some(json_session),
                content: None,
            })
        }
        "jsonl" => {
            let content = export::format_jsonl(&session)?;
            Ok(ExportSessionOutput {
                format: "jsonl".to_string(),
                session: None,
                content: Some(content),
            })
        }
        "markdown" => {
            let content = export::format_markdown(&session)?;
            Ok(ExportSessionOutput {
                format: "markdown".to_string(),
                session: None,
                content: Some(content),
            })
        }
        other => Err(ThinkingError::InvalidInput {
            message: format!("unsupported export format: {}", other),
            field: Some("format".to_string()),
        }),
    }
}
