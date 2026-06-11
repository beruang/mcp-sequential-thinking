use crate::error::ThinkingError;
use crate::model::session::ThinkingSession;

pub fn format_session_jsonl(session: &ThinkingSession) -> Result<String, ThinkingError> {
    let mut output = String::new();
    for thought in &session.thoughts {
        let line = serde_json::to_string(thought)
            .map_err(|e| ThinkingError::SerializationError(e.to_string()))?;
        output.push_str(&line);
        output.push('\n');
    }
    Ok(output)
}
