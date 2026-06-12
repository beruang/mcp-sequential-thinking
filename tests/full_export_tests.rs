use chrono::Utc;
use mcp_sequential_thinking::export;
use mcp_sequential_thinking::model::action::ActionProposal;
use mcp_sequential_thinking::model::branch::{BranchStatus, BranchSummary};
use mcp_sequential_thinking::model::evidence::{EvidenceRef, EvidenceType};
use mcp_sequential_thinking::model::risk::{RiskCategory, RiskInfo, RiskLevel};
use mcp_sequential_thinking::model::session::ThinkingSession;
use mcp_sequential_thinking::model::thought::{ThoughtKind, ThoughtRecord, ThoughtStatus};
use std::collections::HashMap;

fn make_rich_session() -> ThinkingSession {
    let evidence = vec![EvidenceRef {
        evidence_type: EvidenceType::ToolResult,
        source: "context7".into(),
        title: "rmcp docs".into(),
        uri: Some("context7://rmcp/stdio".into()),
        reference: "tool-call-123".into(),
        quote: Some("use serve_server".into()),
        metadata: HashMap::from_iter([("libraryId".into(), "/rust-sdk".into())]),
    }];

    let risk = RiskInfo {
        level: RiskLevel::Medium,
        category: RiskCategory::Operational,
        requires_confirmation: false,
        description: "API may change".into(),
    };

    let action = ActionProposal {
        tool: "context7.get-library-docs".into(),
        args: HashMap::from_iter([("libraryId".into(), serde_json::json!("/rust-sdk"))]),
        risk: Some(RiskInfo {
            level: RiskLevel::Low,
            category: RiskCategory::ExternalSideEffect,
            requires_confirmation: false,
            description: "Side effect via external API".into(),
        }),
        requires_approval: false,
    };

    let t1 = ThoughtRecord {
        thought_id: "th_000001".into(),
        session_id: "rich-session".into(),
        kind: ThoughtKind::Decision,
        content: "Use rmcp for MCP protocol".into(),
        thought_number: 1,
        total_thoughts: 3,
        next_thought_needed: true,
        status: ThoughtStatus::Done,
        confidence: Some(0.9),
        reason_summary: Some("Official Rust MCP SDK".into()),
        branch_id: Some("main".into()),
        branch_label: Some("Main plan".into()),
        branch_from_thought: None,
        branch_status: Some(BranchStatus::Active),
        is_revision: false,
        revises_thought: None,
        depends_on: vec![],
        evidence,
        risk: Some(risk),
        action_proposal: Some(action),
        tags: vec!["rust".into(), "mcp".into()],
        redactions: vec![],
        created_at: Utc::now(),
    };

    let mut session = ThinkingSession::new("rich-session".into());
    session.thoughts = vec![t1];
    session.branches = vec![BranchSummary {
        branch_id: "main".into(),
        branch_label: "Main plan".into(),
        branch_from_thought: None,
        branch_status: BranchStatus::Active,
        thought_count: 1,
    }];
    session
}

#[test]
fn test_markdown_export_with_evidence_risk_action() {
    let session = make_rich_session();
    let result = export::format_markdown(&session).unwrap();
    assert!(result.contains("# Thinking Session rich-session"));
    assert!(result.contains("Use rmcp for MCP protocol"));
    assert!(result.contains("**Confidence:** 0.90"));
    assert!(result.contains("**Evidence:**"));
    assert!(result.contains("rmcp docs"));
    assert!(result.contains("**Risk:** Medium / Operational"));
    assert!(result.contains("**Action proposal:** `context7.get-library-docs`"));
    assert!(result.contains("**Tags:** rust, mcp"));
    assert!(result.contains("**Branch:** main"));
    assert!(result.contains("Status: Active"));
}

#[test]
fn test_jsonl_export_rich() {
    let session = make_rich_session();
    let result = export::format_jsonl(&session).unwrap();
    assert!(result.contains("rich-session"));
    assert!(result.contains("rmcp docs"));
}

#[test]
fn test_json_export_rich() {
    let session = make_rich_session();
    let result = export::format_json(&session).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["sessionId"], "rich-session");
    assert!(!parsed["branches"].as_array().unwrap().is_empty());
}

#[test]
fn test_tool_clear_session_handler_error_paths() {
    use mcp_sequential_thinking::config::AppConfig;
    use mcp_sequential_thinking::store::memory::MemoryStore;
    use mcp_sequential_thinking::tools::clear_session::{handle_clear_session, ClearSessionInput};
    use mcp_sequential_thinking::tools::sequentialthinking::handle_sequentialthinking;
    use mcp_sequential_thinking::validation::rules::ThoughtInput;

    let store = MemoryStore::new(3600, true);
    let config = AppConfig::default();

    // Populate and clear
    let input = ThoughtInput {
        session_id: Some("s1".into()),
        kind: Some("observation".into()),
        content: Some("test".into()),
        thought_number: Some(1),
        total_thoughts: Some(3),
        next_thought_needed: Some(true),
        ..Default::default()
    };
    handle_sequentialthinking(input, &store, &config).unwrap();

    // Clear existing
    let r = handle_clear_session(
        ClearSessionInput {
            session_id: "s1".into(),
        },
        &store,
    )
    .unwrap();
    assert!(r.cleared);

    // Clear again — idempotent
    let r2 = handle_clear_session(
        ClearSessionInput {
            session_id: "s1".into(),
        },
        &store,
    )
    .unwrap();
    assert!(!r2.cleared);
}

#[test]
fn test_tool_get_session_not_found_error() {
    use mcp_sequential_thinking::store::memory::MemoryStore;
    use mcp_sequential_thinking::tools::get_session::{handle_get_session, GetSessionInput};

    let store = MemoryStore::new(3600, true);
    let result = handle_get_session(
        GetSessionInput {
            session_id: "nonexistent".into(),
            include_thoughts: true,
        },
        &store,
    );
    assert!(result.is_err());
}

#[test]
fn test_tool_list_sessions_clamped_limit() {
    use mcp_sequential_thinking::store::memory::MemoryStore;
    use mcp_sequential_thinking::tools::list_sessions::{handle_list_sessions, ListSessionsInput};

    let store = MemoryStore::new(3600, true);
    // limit 0 should be clamped to 1
    let result = handle_list_sessions(ListSessionsInput { limit: 0 }, &store).unwrap();
    assert_eq!(result.sessions.len(), 0);

    // limit 1000 should be clamped to 500
    let _ = handle_list_sessions(ListSessionsInput { limit: 1000 }, &store).unwrap();
}

#[test]
fn test_thought_kind_serde_roundtrip() {
    let kinds = [
        (
            "observation",
            mcp_sequential_thinking::model::thought::ThoughtKind::Observation,
        ),
        (
            "assumption",
            mcp_sequential_thinking::model::thought::ThoughtKind::Assumption,
        ),
        (
            "constraint",
            mcp_sequential_thinking::model::thought::ThoughtKind::Constraint,
        ),
        (
            "risk",
            mcp_sequential_thinking::model::thought::ThoughtKind::Risk,
        ),
        (
            "option",
            mcp_sequential_thinking::model::thought::ThoughtKind::Option,
        ),
        (
            "decision",
            mcp_sequential_thinking::model::thought::ThoughtKind::Decision,
        ),
        (
            "revision",
            mcp_sequential_thinking::model::thought::ThoughtKind::Revision,
        ),
        (
            "validation",
            mcp_sequential_thinking::model::thought::ThoughtKind::Validation,
        ),
        (
            "next_action",
            mcp_sequential_thinking::model::thought::ThoughtKind::NextAction,
        ),
        (
            "blocker",
            mcp_sequential_thinking::model::thought::ThoughtKind::Blocker,
        ),
        (
            "question",
            mcp_sequential_thinking::model::thought::ThoughtKind::Question,
        ),
        (
            "final_decision",
            mcp_sequential_thinking::model::thought::ThoughtKind::FinalDecision,
        ),
    ];
    for (name, kind) in &kinds {
        let serialized = serde_json::to_string(kind).unwrap();
        assert_eq!(serialized, format!("\"{}\"", name));
        let deserialized: mcp_sequential_thinking::model::thought::ThoughtKind =
            serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, *kind);
    }
}

#[test]
fn test_branch_status_serde_roundtrip() {
    let statuses = [
        ("active", BranchStatus::Active),
        ("selected", BranchStatus::Selected),
        ("rejected", BranchStatus::Rejected),
        ("merged", BranchStatus::Merged),
        ("superseded", BranchStatus::Superseded),
    ];
    for (name, status) in &statuses {
        let s = serde_json::to_string(status).unwrap();
        assert_eq!(s, format!("\"{}\"", name));
    }
}

#[test]
fn test_thought_invalid_kind_rejected() {
    use mcp_sequential_thinking::config::AppConfig;
    use mcp_sequential_thinking::store::memory::MemoryStore;
    use mcp_sequential_thinking::tools::sequentialthinking::handle_sequentialthinking;
    use mcp_sequential_thinking::validation::rules::ThoughtInput;

    let store = MemoryStore::new(3600, true);
    let config = AppConfig::default();

    let input = ThoughtInput {
        session_id: Some("s1".into()),
        kind: Some("invalid_kind_xyz".into()),
        content: Some("test".into()),
        thought_number: Some(1),
        total_thoughts: Some(3),
        next_thought_needed: Some(true),
        ..Default::default()
    };
    let result = handle_sequentialthinking(input, &store, &config);
    assert!(result.is_err());
}

#[test]
fn test_reason_summary_length_enforced() {
    use mcp_sequential_thinking::config::AppConfig;
    use mcp_sequential_thinking::store::memory::MemoryStore;
    use mcp_sequential_thinking::tools::sequentialthinking::handle_sequentialthinking;
    use mcp_sequential_thinking::validation::rules::ThoughtInput;

    let store = MemoryStore::new(3600, true);
    let mut config = AppConfig::default();
    config.budgets.max_reason_summary_chars = 5;

    let input = ThoughtInput {
        session_id: Some("s1".into()),
        kind: Some("observation".into()),
        content: Some("test".into()),
        thought_number: Some(1),
        total_thoughts: Some(3),
        next_thought_needed: Some(true),
        reason_summary: Some("This is way too long for summary".into()),
        ..Default::default()
    };
    let result = handle_sequentialthinking(input, &store, &config);
    assert!(result.is_err());
}
