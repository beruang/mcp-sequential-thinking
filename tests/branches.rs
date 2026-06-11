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
fn test_main_branch_default() {
    let store = MemoryStore::new(3600, true);
    let config = AppConfig::default();
    let result =
        handle_sequentialthinking(make_input("task-001", "Thought 1", 1, 5), &store, &config)
            .unwrap();
    assert_eq!(result.branches.len(), 1);
    assert_eq!(result.branches[0].branch_id, "main");
    assert_eq!(result.branches[0].branch_label, "main");
}

#[test]
fn test_new_branch_creation() {
    let store = MemoryStore::new(3600, true);
    let config = AppConfig::default();

    // First thought on main
    handle_sequentialthinking(make_input("task-001", "Thought 1", 1, 5), &store, &config).unwrap();

    // Second thought on a new branch
    let mut input = make_input("task-001", "Alternative approach", 2, 5);
    input.branch_id = Some("alt-approach".to_string());
    input.branch_label = Some("Alternative".to_string());
    let result = handle_sequentialthinking(input, &store, &config).unwrap();

    assert_eq!(result.branches.len(), 2);
    assert!(result.branches.iter().any(|b| b.branch_id == "main"));
    assert!(result
        .branches
        .iter()
        .any(|b| b.branch_id == "alt-approach"));
}

#[test]
fn test_branch_from_thought_validation() {
    let store = MemoryStore::new(3600, true);
    let config = AppConfig::default();
    handle_sequentialthinking(make_input("task-001", "T1", 1, 5), &store, &config).unwrap();

    let mut input = make_input("task-001", "Branching", 2, 5);
    input.branch_id = Some("new-branch".to_string());
    input.branch_from_thought = Some(1);
    let result = handle_sequentialthinking(input, &store, &config);
    assert!(result.is_ok());
}

#[test]
fn test_invalid_branch_reference_rejected() {
    let store = MemoryStore::new(3600, true);
    let config = AppConfig::default();

    let mut input = make_input("task-001", "Bad branch", 1, 5);
    input.branch_id = Some("bad".to_string());
    input.branch_from_thought = Some(999);
    let result = handle_sequentialthinking(input, &store, &config);
    assert!(result.is_err());
}

#[test]
fn test_branch_id_required_when_branch_from_thought_set() {
    let store = MemoryStore::new(3600, true);
    let config = AppConfig::default();
    handle_sequentialthinking(make_input("task-001", "T1", 1, 5), &store, &config).unwrap();

    let mut input = make_input("task-001", "Bad branch", 2, 5);
    input.branch_from_thought = Some(1);
    // branch_id not set
    let result = handle_sequentialthinking(input, &store, &config);
    assert!(result.is_err());
}
