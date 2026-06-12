use chrono::Utc;
use mcp_sequential_thinking::export;
use mcp_sequential_thinking::model::session::ThinkingSession;
use mcp_sequential_thinking::model::thought::{ThoughtKind, ThoughtRecord, ThoughtStatus};

fn make_test_session() -> ThinkingSession {
    let t1 = ThoughtRecord {
        thought_id: "th_000001".into(),
        session_id: "test-session".into(),
        kind: ThoughtKind::Assumption,
        content: "Test content".into(),
        thought_number: 1,
        total_thoughts: 3,
        next_thought_needed: true,
        status: ThoughtStatus::Unverified,
        confidence: Some(0.8),
        reason_summary: Some("Test summary".into()),
        branch_id: Some("main".into()),
        branch_label: None,
        branch_from_thought: None,
        branch_status: None,
        is_revision: false,
        revises_thought: None,
        depends_on: vec![],
        evidence: vec![],
        risk: None,
        action_proposal: None,
        tags: vec!["test".into()],
        redactions: vec![],
        created_at: Utc::now(),
    };
    let mut session = ThinkingSession::new("test-session".into());
    session.thoughts = vec![t1];
    session
}

#[test]
fn test_format_json_direct() {
    let session = make_test_session();
    let result = export::format_json(&session).unwrap();
    assert!(result.contains("test-session"));
    assert!(result.contains("Test content"));
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["sessionId"], "test-session");
}

#[test]
fn test_format_jsonl_direct() {
    let session = make_test_session();
    let result = export::format_jsonl(&session).unwrap();
    assert!(result.contains("test-session"));
    // Each line valid JSON
    for line in result.lines() {
        let _: serde_json::Value = serde_json::from_str(line).unwrap();
    }
}

#[test]
fn test_format_markdown_direct() {
    let session = make_test_session();
    let result = export::format_markdown(&session).unwrap();
    assert!(result.contains("# Thinking Session test-session"));
    assert!(result.contains("Test content"));
    assert!(result.contains("Thought 1"));
    assert!(result.contains("Assumption"));
}

#[test]
fn test_format_markdown_empty_session() {
    let session = ThinkingSession::new("empty".into());
    let result = export::format_markdown(&session).unwrap();
    assert!(result.contains("# Thinking Session empty"));
}

#[test]
fn test_format_jsonl_empty_session() {
    let session = ThinkingSession::new("empty".into());
    let result = export::format_jsonl(&session).unwrap();
    assert!(result.is_empty());
}
