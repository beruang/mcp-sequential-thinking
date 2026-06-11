use mcp_sequential_thinking::config::AppConfig;
use mcp_sequential_thinking::store::memory::MemoryStore;
use mcp_sequential_thinking::tools::sequentialthinking::handle_sequentialthinking;
use mcp_sequential_thinking::validation::rules::ThoughtInput;

fn make_input(session_id: &str, content: &str, num: u32, total: u32) -> ThoughtInput {
    ThoughtInput {
        session_id: Some(session_id.to_string()),
        kind: Some("assumption".to_string()),
        content: Some(content.to_string()),
        thought_number: Some(num),
        total_thoughts: Some(total),
        next_thought_needed: Some(true),
        ..Default::default()
    }
}

#[test]
fn test_max_thoughts_enforced() {
    let store = MemoryStore::new(3600, true);
    let mut config = AppConfig::default();
    config.budgets.max_thoughts_per_session = 3;

    handle_sequentialthinking(make_input("task-001", "1", 1, 10), &store, &config).unwrap();
    handle_sequentialthinking(make_input("task-001", "2", 2, 10), &store, &config).unwrap();
    handle_sequentialthinking(make_input("task-001", "3", 3, 10), &store, &config).unwrap();

    let result = handle_sequentialthinking(make_input("task-001", "4", 4, 10), &store, &config);
    assert!(result.is_err());
}

#[test]
fn test_final_decision_allowed_after_budget() {
    let store = MemoryStore::new(3600, true);
    let mut config = AppConfig::default();
    config.budgets.max_thoughts_per_session = 2;

    handle_sequentialthinking(make_input("task-001", "1", 1, 5), &store, &config).unwrap();
    handle_sequentialthinking(make_input("task-001", "2", 2, 5), &store, &config).unwrap();

    let mut input = make_input("task-001", "Final", 3, 5);
    input.kind = Some("final_decision".to_string());
    let result = handle_sequentialthinking(input, &store, &config);
    assert!(result.is_ok());
}

#[test]
fn test_content_length_enforced() {
    let store = MemoryStore::new(3600, true);
    let mut config = AppConfig::default();
    config.budgets.max_content_chars = 10;

    let mut input = make_input(
        "task-001",
        "This content is way too long for the budget limit",
        1,
        5,
    );
    // Make content actually shorter but still exceeding 10 chars
    input.content = Some("This is more than 10 characters long".to_string());
    let result = handle_sequentialthinking(input, &store, &config);
    assert!(result.is_err());
}

#[test]
fn test_max_branches_enforced() {
    let store = MemoryStore::new(3600, true);
    let mut config = AppConfig::default();
    config.budgets.max_branches_per_session = 2;

    handle_sequentialthinking(make_input("task-001", "1", 1, 10), &store, &config).unwrap();

    let mut b1 = make_input("task-001", "b1", 2, 10);
    b1.branch_id = Some("b1".to_string());
    handle_sequentialthinking(b1, &store, &config).unwrap();

    // Third branch (main + b1 = 2 already) should be rejected
    let mut b2 = make_input("task-001", "b2", 3, 10);
    b2.branch_id = Some("b2".to_string());
    let result = handle_sequentialthinking(b2, &store, &config);
    assert!(result.is_err());
}

#[test]
fn test_evidence_count_enforced() {
    let store = MemoryStore::new(3600, true);
    let mut config = AppConfig::default();
    config.budgets.max_evidence_items_per_thought = 2;

    let mut input = make_input("task-001", "T1", 1, 5);
    input.evidence = Some(vec![
        mcp_sequential_thinking::model::evidence::EvidenceRef {
            evidence_type: mcp_sequential_thinking::model::evidence::EvidenceType::ToolResult,
            source: "test".into(),
            title: "ev1".into(),
            uri: None,
            reference: "ref1".into(),
            quote: None,
            metadata: Default::default(),
        },
        mcp_sequential_thinking::model::evidence::EvidenceRef {
            evidence_type: mcp_sequential_thinking::model::evidence::EvidenceType::ToolResult,
            source: "test".into(),
            title: "ev2".into(),
            uri: None,
            reference: "ref2".into(),
            quote: None,
            metadata: Default::default(),
        },
        mcp_sequential_thinking::model::evidence::EvidenceRef {
            evidence_type: mcp_sequential_thinking::model::evidence::EvidenceType::ToolResult,
            source: "test".into(),
            title: "ev3".into(),
            uri: None,
            reference: "ref3".into(),
            quote: None,
            metadata: Default::default(),
        },
    ]);
    let result = handle_sequentialthinking(input, &store, &config);
    assert!(result.is_err());
}

#[test]
fn test_tag_count_enforced() {
    let store = MemoryStore::new(3600, true);
    let mut config = AppConfig::default();
    config.budgets.max_tags_per_thought = 3;

    let mut input = make_input("task-001", "T1", 1, 5);
    input.tags = Some(vec!["a".into(), "b".into(), "c".into(), "d".into()]);
    let result = handle_sequentialthinking(input, &store, &config);
    assert!(result.is_err());
}

#[test]
fn test_invalid_confidence_rejected() {
    let store = MemoryStore::new(3600, true);
    let config = AppConfig::default();

    let mut input = make_input("task-001", "T1", 1, 5);
    input.confidence = Some(1.5);
    let result = handle_sequentialthinking(input, &store, &config);
    assert!(result.is_err());

    let mut input2 = make_input("task-001", "T2", 1, 5);
    input2.confidence = Some(-0.1);
    let result2 = handle_sequentialthinking(input2, &store, &config);
    assert!(result2.is_err());
}
