use mcp_sequential_thinking::config::AppConfig;
use mcp_sequential_thinking::model::thought::ThoughtKind;
use mcp_sequential_thinking::store::memory::MemoryStore;
use mcp_sequential_thinking::store::ThinkingStore;
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
fn test_revision_requires_existing_thought() {
    let store = MemoryStore::new(3600, true);
    let config = AppConfig::default();
    handle_sequentialthinking(make_input("task-001", "T1", 1, 5), &store, &config).unwrap();

    let mut input = make_input("task-001", "Revised", 2, 5);
    input.kind = Some("revision".to_string());
    input.revises_thought = Some(1);
    let result = handle_sequentialthinking(input, &store, &config);
    assert!(result.is_ok());
}

#[test]
fn test_revision_auto_supersedes_target() {
    let store = MemoryStore::new(3600, true);
    let config = AppConfig::default();
    handle_sequentialthinking(make_input("task-001", "T1", 1, 5), &store, &config).unwrap();

    let mut input = make_input("task-001", "Revised", 2, 5);
    input.kind = Some("revision".to_string());
    input.revises_thought = Some(1);
    handle_sequentialthinking(input, &store, &config).unwrap();

    let session = store.get_session("task-001").unwrap().unwrap();
    let t1 = session
        .thoughts
        .iter()
        .find(|t| t.thought_number == 1)
        .unwrap();
    assert_eq!(
        t1.status,
        mcp_sequential_thinking::model::thought::ThoughtStatus::Superseded
    );
}

#[test]
fn test_invalid_revision_reference_rejected() {
    let store = MemoryStore::new(3600, true);
    let config = AppConfig::default();

    let mut input = make_input("task-001", "Bad revision", 1, 5);
    input.kind = Some("revision".to_string());
    input.revises_thought = Some(999);
    let result = handle_sequentialthinking(input, &store, &config);
    assert!(result.is_err());
}

#[test]
fn test_kind_revision_sets_is_revision_true() {
    let store = MemoryStore::new(3600, true);
    let config = AppConfig::default();
    handle_sequentialthinking(make_input("task-001", "T1", 1, 5), &store, &config).unwrap();

    let mut input = make_input("task-001", "Correction", 2, 5);
    input.kind = Some("revision".to_string());
    input.revises_thought = Some(1);
    let _output = handle_sequentialthinking(input, &store, &config).unwrap();

    let session = store.get_session("task-001").unwrap().unwrap();
    let thought = session
        .thoughts
        .iter()
        .find(|t| t.thought_number == 2)
        .unwrap();
    assert!(thought.is_revision);
    assert_eq!(thought.kind, ThoughtKind::Revision);
}

#[test]
fn test_is_revision_true_normalizes_kind_to_revision() {
    let store = MemoryStore::new(3600, true);
    let config = AppConfig::default();
    handle_sequentialthinking(make_input("task-001", "T1", 1, 5), &store, &config).unwrap();

    let mut input = make_input("task-001", "Fake revision", 2, 5);
    input.is_revision = Some(true);
    input.revises_thought = Some(1);
    let output = handle_sequentialthinking(input, &store, &config).unwrap();

    assert!(output
        .warnings
        .iter()
        .any(|w| w.code == "kind_normalized_to_revision"));
}
