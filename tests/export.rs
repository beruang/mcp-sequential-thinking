use mcp_sequential_thinking::config::AppConfig;
use mcp_sequential_thinking::store::memory::MemoryStore;
use mcp_sequential_thinking::tools::export_session::handle_export_session;
use mcp_sequential_thinking::tools::export_session::ExportSessionInput;
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
fn test_json_export() {
    let store = MemoryStore::new(3600, true);
    let config = AppConfig::default();
    handle_sequentialthinking(make_input("task-001", "First", 1, 5), &store, &config).unwrap();
    handle_sequentialthinking(make_input("task-001", "Second", 2, 5), &store, &config).unwrap();

    let result = handle_export_session(
        ExportSessionInput {
            session_id: "task-001".to_string(),
            format: "json".to_string(),
        },
        &store,
    )
    .unwrap();

    assert_eq!(result.format, "json");
    assert!(result.session.is_some());
    let session = result.session.unwrap();
    assert_eq!(session["sessionId"], "task-001");
    assert_eq!(session["thoughts"].as_array().unwrap().len(), 2);
}

#[test]
fn test_jsonl_export() {
    let store = MemoryStore::new(3600, true);
    let config = AppConfig::default();
    handle_sequentialthinking(make_input("task-001", "First", 1, 5), &store, &config).unwrap();

    let result = handle_export_session(
        ExportSessionInput {
            session_id: "task-001".to_string(),
            format: "jsonl".to_string(),
        },
        &store,
    )
    .unwrap();

    assert_eq!(result.format, "jsonl");
    assert!(result.content.is_some());
    let content = result.content.unwrap();
    assert!(content.contains("First"));
    // Should be valid JSON per line
    for line in content.lines() {
        let _: serde_json::Value = serde_json::from_str(line).unwrap();
    }
}

#[test]
fn test_markdown_export() {
    let store = MemoryStore::new(3600, true);
    let config = AppConfig::default();
    handle_sequentialthinking(
        make_input("task-001", "Reasoning step", 1, 5),
        &store,
        &config,
    )
    .unwrap();

    let result = handle_export_session(
        ExportSessionInput {
            session_id: "task-001".to_string(),
            format: "markdown".to_string(),
        },
        &store,
    )
    .unwrap();

    assert_eq!(result.format, "markdown");
    assert!(result.content.is_some());
    let content = result.content.unwrap();
    assert!(content.contains("# Thinking Session task-001"));
    assert!(content.contains("Reasoning step"));
    assert!(content.contains("Thought 1"));
}

#[test]
fn test_export_missing_session_error() {
    let store = MemoryStore::new(3600, true);
    let result = handle_export_session(
        ExportSessionInput {
            session_id: "missing".to_string(),
            format: "json".to_string(),
        },
        &store,
    );
    assert!(result.is_err());
}

#[test]
fn test_invalid_export_format_error() {
    let store = MemoryStore::new(3600, true);
    let config = AppConfig::default();
    handle_sequentialthinking(make_input("task-001", "T1", 1, 5), &store, &config).unwrap();

    let result = handle_export_session(
        ExportSessionInput {
            session_id: "task-001".to_string(),
            format: "xml".to_string(),
        },
        &store,
    );
    assert!(result.is_err());
}
