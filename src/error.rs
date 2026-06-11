use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCode {
    InvalidInput,
    InvalidKind,
    InvalidStatus,
    InvalidConfidence,
    MissingSessionId,
    SessionNotFound,
    BudgetExceeded,
    ContentTooLarge,
    InvalidBranch,
    InvalidRevision,
    InvalidDependency,
    RetentionExpired,
    RedactionFailed,
    SerializationError,
    InternalError,
}

#[derive(Debug, Error)]
pub enum ThinkingError {
    #[error("invalid input: {message}")]
    InvalidInput {
        message: String,
        field: Option<String>,
    },

    #[error("invalid kind: {0}")]
    InvalidKind(String),

    #[error("invalid status: {0}")]
    InvalidStatus(String),

    #[error("confidence must be between 0.0 and 1.0")]
    InvalidConfidence,

    #[error("missing session ID")]
    MissingSessionId,

    #[error("session not found: {0}")]
    SessionNotFound(String),

    #[error("budget exceeded: {0}")]
    BudgetExceeded(String),

    #[error("content too large: {0}")]
    ContentTooLarge(String),

    #[error("invalid branch: {0}")]
    InvalidBranch(String),

    #[error("invalid revision: {0}")]
    InvalidRevision(String),

    #[error("invalid dependency: {0}")]
    InvalidDependency(String),

    #[error("retention expired")]
    RetentionExpired,

    #[error("redaction failed: {0}")]
    RedactionFailed(String),

    #[error("serialization error: {0}")]
    SerializationError(String),

    #[error("internal error: {0}")]
    InternalError(String),
}

impl ThinkingError {
    pub fn code(&self) -> ErrorCode {
        match self {
            Self::InvalidInput { .. } => ErrorCode::InvalidInput,
            Self::InvalidKind(_) => ErrorCode::InvalidKind,
            Self::InvalidStatus(_) => ErrorCode::InvalidStatus,
            Self::InvalidConfidence => ErrorCode::InvalidConfidence,
            Self::MissingSessionId => ErrorCode::MissingSessionId,
            Self::SessionNotFound(_) => ErrorCode::SessionNotFound,
            Self::BudgetExceeded(_) => ErrorCode::BudgetExceeded,
            Self::ContentTooLarge(_) => ErrorCode::ContentTooLarge,
            Self::InvalidBranch(_) => ErrorCode::InvalidBranch,
            Self::InvalidRevision(_) => ErrorCode::InvalidRevision,
            Self::InvalidDependency(_) => ErrorCode::InvalidDependency,
            Self::RetentionExpired => ErrorCode::RetentionExpired,
            Self::RedactionFailed(_) => ErrorCode::RedactionFailed,
            Self::SerializationError(_) => ErrorCode::SerializationError,
            Self::InternalError(_) => ErrorCode::InternalError,
        }
    }

    pub fn to_mcp_error(&self) -> serde_json::Value {
        let mut map = serde_json::Map::new();
        map.insert(
            "code".to_string(),
            serde_json::Value::String(
                serde_json::to_string(&self.code())
                    .unwrap_or_default()
                    .trim_matches('"')
                    .to_string(),
            ),
        );
        map.insert(
            "message".to_string(),
            serde_json::Value::String(self.to_string()),
        );
        if let Self::InvalidInput {
            field: Some(ref f), ..
        } = self
        {
            map.insert("field".to_string(), serde_json::Value::String(f.clone()));
        }
        serde_json::Value::Object(map)
    }
}
