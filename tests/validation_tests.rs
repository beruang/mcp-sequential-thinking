use mcp_sequential_thinking::model::thought::{ThoughtKind, ThoughtStatus};
use mcp_sequential_thinking::validation::defaults::default_status_for_kind;

#[test]
fn test_all_default_statuses() {
    assert_eq!(
        default_status_for_kind(ThoughtKind::Observation),
        ThoughtStatus::Verified
    );
    assert_eq!(
        default_status_for_kind(ThoughtKind::Assumption),
        ThoughtStatus::Unverified
    );
    assert_eq!(
        default_status_for_kind(ThoughtKind::Constraint),
        ThoughtStatus::Active
    );
    assert_eq!(
        default_status_for_kind(ThoughtKind::Risk),
        ThoughtStatus::Active
    );
    assert_eq!(
        default_status_for_kind(ThoughtKind::Option),
        ThoughtStatus::Active
    );
    assert_eq!(
        default_status_for_kind(ThoughtKind::Decision),
        ThoughtStatus::Done
    );
    assert_eq!(
        default_status_for_kind(ThoughtKind::Revision),
        ThoughtStatus::Done
    );
    assert_eq!(
        default_status_for_kind(ThoughtKind::Validation),
        ThoughtStatus::Done
    );
    assert_eq!(
        default_status_for_kind(ThoughtKind::NextAction),
        ThoughtStatus::Active
    );
    assert_eq!(
        default_status_for_kind(ThoughtKind::Blocker),
        ThoughtStatus::Blocked
    );
    assert_eq!(
        default_status_for_kind(ThoughtKind::Question),
        ThoughtStatus::Active
    );
    assert_eq!(
        default_status_for_kind(ThoughtKind::FinalDecision),
        ThoughtStatus::Done
    );
}

#[test]
fn test_error_code_to_string() {
    use mcp_sequential_thinking::error::ThinkingError;

    let err = ThinkingError::InvalidConfidence;
    let code = err.code();
    assert_eq!(format!("{:?}", code), "InvalidConfidence");

    let err = ThinkingError::SessionNotFound("test".into());
    assert_eq!(format!("{}", err), "session not found: test");

    let err = ThinkingError::InvalidInput {
        message: "bad".into(),
        field: Some("f".into()),
    };
    assert_eq!(format!("{}", err), "invalid input: bad");
    let json = err.to_mcp_error();
    assert_eq!(json["code"], "invalid_input");
    assert_eq!(json["field"], "f");

    // Test all error codes
    let errors = vec![
        ThinkingError::InvalidInput {
            message: "x".into(),
            field: None,
        },
        ThinkingError::InvalidKind("x".into()),
        ThinkingError::InvalidStatus("x".into()),
        ThinkingError::InvalidConfidence,
        ThinkingError::MissingSessionId,
        ThinkingError::SessionNotFound("x".into()),
        ThinkingError::BudgetExceeded("x".into()),
        ThinkingError::ContentTooLarge("x".into()),
        ThinkingError::InvalidBranch("x".into()),
        ThinkingError::InvalidRevision("x".into()),
        ThinkingError::InvalidDependency("x".into()),
        ThinkingError::RetentionExpired,
        ThinkingError::RedactionFailed("x".into()),
        ThinkingError::SerializationError("x".into()),
        ThinkingError::InternalError("x".into()),
    ];
    for err in &errors {
        let json = err.to_mcp_error();
        assert!(json["code"].is_string());
        assert!(json["message"].is_string());
    }
}
