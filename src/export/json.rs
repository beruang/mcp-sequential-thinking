use crate::error::ThinkingError;
use crate::model::session::ThinkingSession;

pub fn format_session_json(session: &ThinkingSession) -> Result<String, ThinkingError> {
    let output = serde_json::to_string_pretty(session)
        .map_err(|e| ThinkingError::SerializationError(e.to_string()))?;
    Ok(output)
}
