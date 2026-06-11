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
fn test_bearer_token_redacted() {
    let store = MemoryStore::new(3600, true);
    let config = AppConfig::default();

    let input = make_input("task-001", "Auth: Bearer abc123def456", 1, 5);
    let _result = handle_sequentialthinking(input, &store, &config).unwrap();

    let session = store.get_session("task-001").unwrap().unwrap();
    assert!(!session.thoughts[0].content.contains("Bearer abc123def456"));
    assert!(session.thoughts[0]
        .content
        .contains("[REDACTED:bearer_token]"));
}

#[test]
fn test_api_key_redacted() {
    let store = MemoryStore::new(3600, true);
    let config = AppConfig::default();

    let input = make_input("task-001", "api_key=sk-mysecretkey123", 1, 5);
    let _result = handle_sequentialthinking(input, &store, &config).unwrap();

    let session = store.get_session("task-001").unwrap().unwrap();
    assert!(session.thoughts[0].content.contains("[REDACTED:api_key]"));
}

#[test]
fn test_github_token_redacted() {
    let store = MemoryStore::new(3600, true);
    let config = AppConfig::default();

    let input = make_input("task-001", "ghp_123456789012345678901234567890123456", 1, 5);
    let _result = handle_sequentialthinking(input, &store, &config).unwrap();

    let session = store.get_session("task-001").unwrap().unwrap();
    assert!(session.thoughts[0]
        .content
        .contains("[REDACTED:github_token]"));
}

#[test]
fn test_aws_key_redacted() {
    let store = MemoryStore::new(3600, true);
    let config = AppConfig::default();

    let input = make_input("task-001", "AKIA1234567890ABCDEF", 1, 5);
    let _result = handle_sequentialthinking(input, &store, &config).unwrap();

    let session = store.get_session("task-001").unwrap().unwrap();
    assert!(!session.thoughts[0].content.contains("AKIA"));
}

#[test]
fn test_jwt_redacted() {
    let store = MemoryStore::new(3600, true);
    let config = AppConfig::default();

    let input = make_input(
        "task-001",
        "eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIxMjM0NTY3ODkwIn0.dozjgNqP_WR1sYc",
        1,
        5,
    );
    let _result = handle_sequentialthinking(input, &store, &config).unwrap();

    let session = store.get_session("task-001").unwrap().unwrap();
    assert!(!session.thoughts[0].content.contains("eyJ"));
}

#[test]
fn test_no_redaction_when_disabled() {
    let store = MemoryStore::new(3600, true);
    let mut config = AppConfig::default();
    config.redaction.enabled = false;

    let input = make_input("task-001", "Bearer abc123", 1, 5);
    let _result = handle_sequentialthinking(input, &store, &config).unwrap();

    let session = store.get_session("task-001").unwrap().unwrap();
    assert!(session.thoughts[0].content.contains("Bearer abc123"));
}

#[test]
fn test_redactions_in_output() {
    let store = MemoryStore::new(3600, true);
    let config = AppConfig::default();

    let input = make_input("task-001", "My token: Bearer abc123", 1, 5);
    let result = handle_sequentialthinking(input, &store, &config).unwrap();
    assert!(!result.redactions.is_empty());
    assert!(result.redactions.iter().any(|r| r.kind == "bearer_token"));
}
