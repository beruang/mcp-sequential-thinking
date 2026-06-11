use mcp_sequential_thinking::config::AppConfig;
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
fn test_missing_dependency_rejected() {
    let store = MemoryStore::new(3600, true);
    let config = AppConfig::default();
    handle_sequentialthinking(make_input("task-001", "T1", 1, 5), &store, &config).unwrap();

    let mut input = make_input("task-001", "T2", 2, 5);
    input.depends_on = Some(vec![999]);
    let result = handle_sequentialthinking(input, &store, &config);
    assert!(result.is_err());
}

#[test]
fn test_future_dependency_rejected() {
    let store = MemoryStore::new(3600, true);
    let config = AppConfig::default();

    let mut input = make_input("task-001", "T1", 1, 5);
    input.depends_on = Some(vec![5]); // future thought
    let result = handle_sequentialthinking(input, &store, &config);
    assert!(result.is_err());
}

#[test]
fn test_self_dependency_rejected() {
    let store = MemoryStore::new(3600, true);
    let config = AppConfig::default();

    let mut input = make_input("task-001", "T1", 1, 5);
    input.depends_on = Some(vec![1]); // self
    let result = handle_sequentialthinking(input, &store, &config);
    assert!(result.is_err());
}

#[test]
fn test_duplicate_dependencies_deduplicated() {
    let store = MemoryStore::new(3600, true);
    let config = AppConfig::default();
    handle_sequentialthinking(make_input("task-001", "T1", 1, 5), &store, &config).unwrap();

    let mut input = make_input("task-001", "T2", 2, 5);
    input.depends_on = Some(vec![1, 1, 1]);
    let _result = handle_sequentialthinking(input, &store, &config).unwrap();
    let session = store.get_session("task-001").unwrap().unwrap();
    let thought = session
        .thoughts
        .iter()
        .find(|t| t.thought_number == 2)
        .unwrap();
    assert_eq!(thought.depends_on.len(), 1);
}

#[test]
fn test_valid_dependency_accepted() {
    let store = MemoryStore::new(3600, true);
    let config = AppConfig::default();
    handle_sequentialthinking(make_input("task-001", "T1", 1, 5), &store, &config).unwrap();

    let mut input = make_input("task-001", "T2 depends on T1", 2, 5);
    input.depends_on = Some(vec![1]);
    let result = handle_sequentialthinking(input, &store, &config);
    assert!(result.is_ok());
}
