use mcp_sequential_thinking::config::AppConfig;
use mcp_sequential_thinking::store::memory::MemoryStore;
use mcp_sequential_thinking::tools::clear_session::handle_clear_session;
use mcp_sequential_thinking::tools::clear_session::ClearSessionInput;
use mcp_sequential_thinking::tools::get_session::handle_get_session;
use mcp_sequential_thinking::tools::get_session::GetSessionInput;
use mcp_sequential_thinking::tools::list_sessions::handle_list_sessions;
use mcp_sequential_thinking::tools::list_sessions::ListSessionsInput;
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
fn test_new_session_creation() {
    let store = MemoryStore::new(3600, true);
    let config = AppConfig::default();
    let input = make_input("task-001", "First thought", 1, 5);
    let result = handle_sequentialthinking(input, &store, &config).unwrap();
    assert!(result.accepted);
    assert_eq!(result.session_id, "task-001");
    assert_eq!(result.thought_history_length, 1);
}

#[test]
fn test_session_retrieval() {
    let store = MemoryStore::new(3600, true);
    let config = AppConfig::default();
    handle_sequentialthinking(make_input("task-001", "Thought 1", 1, 5), &store, &config).unwrap();

    let session = handle_get_session(
        GetSessionInput {
            session_id: "task-001".to_string(),
            include_thoughts: true,
        },
        &store,
    )
    .unwrap();
    assert_eq!(session.session_id, "task-001");
    assert_eq!(session.thought_count, 1);
    assert!(session.thoughts.is_some());
}

#[test]
fn test_session_retrieval_without_thoughts() {
    let store = MemoryStore::new(3600, true);
    let config = AppConfig::default();
    handle_sequentialthinking(make_input("task-001", "Thought 1", 1, 5), &store, &config).unwrap();

    let session = handle_get_session(
        GetSessionInput {
            session_id: "task-001".to_string(),
            include_thoughts: false,
        },
        &store,
    )
    .unwrap();
    assert!(session.thoughts.is_none());
}

#[test]
fn test_session_listing() {
    let store = MemoryStore::new(3600, true);
    let config = AppConfig::default();
    handle_sequentialthinking(make_input("task-001", "T1", 1, 3), &store, &config).unwrap();
    handle_sequentialthinking(make_input("task-002", "T1", 1, 3), &store, &config).unwrap();

    let list = handle_list_sessions(ListSessionsInput { limit: 50 }, &store).unwrap();
    assert_eq!(list.sessions.len(), 2);
}

#[test]
fn test_session_clearing() {
    let store = MemoryStore::new(3600, true);
    let config = AppConfig::default();
    handle_sequentialthinking(make_input("task-001", "T1", 1, 3), &store, &config).unwrap();

    let result = handle_clear_session(
        ClearSessionInput {
            session_id: "task-001".to_string(),
        },
        &store,
    )
    .unwrap();
    assert!(result.cleared);
}

#[test]
fn test_idempotent_clearing() {
    let store = MemoryStore::new(3600, true);
    let result = handle_clear_session(
        ClearSessionInput {
            session_id: "missing".to_string(),
        },
        &store,
    )
    .unwrap();
    assert!(!result.cleared);
}
