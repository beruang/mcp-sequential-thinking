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
fn test_ttl_expiration() {
    // Use very short TTL
    let store = MemoryStore::new(0, true);
    let config = AppConfig::default();
    handle_sequentialthinking(make_input("task-001", "T1", 1, 5), &store, &config).unwrap();

    // With TTL=0, the session should be expired immediately
    let session = store.get_session("task-001").unwrap();
    assert!(session.is_none());
}

#[test]
fn test_cleanup_removes_expired() {
    let store = MemoryStore::new(0, true);
    let config = AppConfig::default();
    handle_sequentialthinking(make_input("task-001", "T1", 1, 5), &store, &config).unwrap();

    let removed = store.cleanup_expired().unwrap();
    assert_eq!(removed, 1);
}

#[test]
fn test_disabled_retention_skips_ttl() {
    let store = MemoryStore::new(0, false); // 0 TTL but retention disabled
    let config = AppConfig::default();
    handle_sequentialthinking(make_input("task-001", "T1", 1, 5), &store, &config).unwrap();

    let session = store.get_session("task-001").unwrap();
    assert!(session.is_some());
}

#[test]
fn test_cleanup_does_not_remove_fresh_sessions() {
    let store = MemoryStore::new(3600, true);
    let config = AppConfig::default();
    handle_sequentialthinking(make_input("task-001", "T1", 1, 5), &store, &config).unwrap();

    let removed = store.cleanup_expired().unwrap();
    assert_eq!(removed, 0);
}
