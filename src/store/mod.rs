use crate::error::ThinkingError;
use crate::model::session::{SessionSummary, ThinkingSession};
use crate::model::thought::ThoughtRecord;

pub mod memory;
pub mod retention;

pub trait ThinkingStore: Send + Sync + 'static {
    fn upsert_thought(&self, thought: ThoughtRecord) -> Result<ThoughtRecord, ThinkingError>;

    fn get_session(&self, session_id: &str) -> Result<Option<ThinkingSession>, ThinkingError>;

    fn list_sessions(&self, limit: usize) -> Result<Vec<SessionSummary>, ThinkingError>;

    fn clear_session(&self, session_id: &str) -> Result<bool, ThinkingError>;

    fn cleanup_expired(&self) -> Result<usize, ThinkingError>;
}
