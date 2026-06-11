use crate::error::ThinkingError;
use crate::model::session::ThinkingSession;

mod json;
mod jsonl;
mod markdown;

pub fn format_json(session: &ThinkingSession) -> Result<String, ThinkingError> {
    json::format_session_json(session)
}

pub fn format_jsonl(session: &ThinkingSession) -> Result<String, ThinkingError> {
    jsonl::format_session_jsonl(session)
}

pub fn format_markdown(session: &ThinkingSession) -> Result<String, ThinkingError> {
    markdown::format_session_markdown(session)
}
