use mcp_sequential_thinking::config::AppConfig;
use mcp_sequential_thinking::model::thought::ThoughtKind;
use mcp_sequential_thinking::store::memory::MemoryStore;
use mcp_sequential_thinking::store::ThinkingStore;
use mcp_sequential_thinking::tools::sequentialthinking::handle_sequentialthinking;
use mcp_sequential_thinking::validation::rules::ThoughtInput;

fn make_legacy_input(thought: &str, num: u32, total: u32) -> ThoughtInput {
    ThoughtInput {
        thought: Some(thought.to_string()),
        content: None,
        kind: None,
        session_id: None,
        thought_number: Some(num),
        total_thoughts: Some(total),
        next_thought_needed: Some(true),
        ..Default::default()
    }
}

#[test]
fn test_legacy_input_accepted() {
    let store = MemoryStore::new(3600, true);
    let config = AppConfig::default();
    let input = make_legacy_input("We need to inspect the problem first.", 1, 5);
    let result = handle_sequentialthinking(input, &store, &config).unwrap();
    assert!(result.accepted);
    assert!(result.warnings.iter().any(|w| w.code == "legacy_input"));
}

#[test]
fn test_legacy_input_maps_to_default_session() {
    let store = MemoryStore::new(3600, true);
    let config = AppConfig::default();
    let input = make_legacy_input("legacy thought", 1, 3);
    let result = handle_sequentialthinking(input, &store, &config).unwrap();
    assert_eq!(result.session_id, "default");
}

#[test]
fn test_legacy_input_maps_to_observation_kind() {
    let store = MemoryStore::new(3600, true);
    let config = AppConfig::default();
    let input = make_legacy_input("legacy thought", 1, 3);
    let _output = handle_sequentialthinking(input, &store, &config).unwrap();
    let session = store.get_session("default").unwrap().unwrap();
    let thought = &session.thoughts[0];
    assert_eq!(thought.kind, ThoughtKind::Observation);
}
