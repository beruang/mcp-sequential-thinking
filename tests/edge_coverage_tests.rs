#[test]
fn test_list_sessions_max_boundary_clamped() {
    use mcp_sequential_thinking::config::AppConfig;
    use mcp_sequential_thinking::store::memory::MemoryStore;
    use mcp_sequential_thinking::tools::list_sessions::{handle_list_sessions, ListSessionsInput};
    use mcp_sequential_thinking::tools::sequentialthinking::handle_sequentialthinking;
    use mcp_sequential_thinking::validation::rules::ThoughtInput;

    let store = MemoryStore::new(3600, true);
    let config = AppConfig::default();

    for i in 0..5 {
        let input = ThoughtInput {
            session_id: Some(format!("s{}", i)),
            kind: Some("observation".into()),
            content: Some("test".into()),
            thought_number: Some(1),
            total_thoughts: Some(3),
            next_thought_needed: Some(true),
            ..Default::default()
        };
        handle_sequentialthinking(input, &store, &config).unwrap();
    }

    // Test default limit (50)
    let r1 = handle_list_sessions(ListSessionsInput { limit: 50 }, &store).unwrap();
    assert_eq!(r1.sessions.len(), 5);

    // Test max boundary — limit: 500 is max
    let r2 = handle_list_sessions(ListSessionsInput { limit: 500 }, &store).unwrap();
    assert_eq!(r2.sessions.len(), 5);

    // Test exceeding max is clamped
    let r3 = handle_list_sessions(ListSessionsInput { limit: 501 }, &store).unwrap();
    assert_eq!(r3.sessions.len(), 5);
}

#[test]
fn test_sequentialthinking_terminal_thought_warning() {
    use mcp_sequential_thinking::config::AppConfig;
    use mcp_sequential_thinking::store::memory::MemoryStore;
    use mcp_sequential_thinking::tools::sequentialthinking::handle_sequentialthinking;
    use mcp_sequential_thinking::validation::rules::ThoughtInput;

    let store = MemoryStore::new(3600, true);
    let config = AppConfig::default();

    let input = ThoughtInput {
        session_id: Some("s1".into()),
        kind: Some("observation".into()),
        content: Some("final thought".into()),
        thought_number: Some(1),
        total_thoughts: Some(3),
        next_thought_needed: Some(false),
        ..Default::default()
    };
    let result = handle_sequentialthinking(input, &store, &config).unwrap();
    assert!(result
        .warnings
        .iter()
        .any(|w| w.code == "non_final_terminal_thought"));
}

#[test]
fn test_empty_session_id_rejected() {
    use mcp_sequential_thinking::config::AppConfig;
    use mcp_sequential_thinking::store::memory::MemoryStore;
    use mcp_sequential_thinking::tools::sequentialthinking::handle_sequentialthinking;
    use mcp_sequential_thinking::validation::rules::ThoughtInput;

    let store = MemoryStore::new(3600, true);
    let config = AppConfig::default();

    // Empty string session ID
    let input = ThoughtInput {
        session_id: Some("".into()),
        kind: Some("observation".into()),
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
fn test_thought_number_zero_rejected() {
    use mcp_sequential_thinking::config::AppConfig;
    use mcp_sequential_thinking::store::memory::MemoryStore;
    use mcp_sequential_thinking::tools::sequentialthinking::handle_sequentialthinking;
    use mcp_sequential_thinking::validation::rules::ThoughtInput;

    let store = MemoryStore::new(3600, true);
    let config = AppConfig::default();

    let input = ThoughtInput {
        session_id: Some("s1".into()),
        kind: Some("observation".into()),
        content: Some("test".into()),
        thought_number: Some(0),
        total_thoughts: Some(3),
        next_thought_needed: Some(true),
        ..Default::default()
    };
    let result = handle_sequentialthinking(input, &store, &config);
    assert!(result.is_err());
}

#[test]
fn test_total_thoughts_zero_rejected() {
    use mcp_sequential_thinking::config::AppConfig;
    use mcp_sequential_thinking::store::memory::MemoryStore;
    use mcp_sequential_thinking::tools::sequentialthinking::handle_sequentialthinking;
    use mcp_sequential_thinking::validation::rules::ThoughtInput;

    let store = MemoryStore::new(3600, true);
    let config = AppConfig::default();

    let input = ThoughtInput {
        session_id: Some("s1".into()),
        kind: Some("observation".into()),
        content: Some("test".into()),
        thought_number: Some(1),
        total_thoughts: Some(0),
        next_thought_needed: Some(true),
        ..Default::default()
    };
    let result = handle_sequentialthinking(input, &store, &config);
    assert!(result.is_err());
}

#[test]
fn test_default_confidence_accepted() {
    use mcp_sequential_thinking::config::AppConfig;
    use mcp_sequential_thinking::store::memory::MemoryStore;
    use mcp_sequential_thinking::tools::sequentialthinking::handle_sequentialthinking;
    use mcp_sequential_thinking::validation::rules::ThoughtInput;

    let store = MemoryStore::new(3600, true);
    let config = AppConfig::default();

    let input = ThoughtInput {
        session_id: Some("s1".into()),
        kind: Some("observation".into()),
        content: Some("test".into()),
        thought_number: Some(1),
        total_thoughts: Some(3),
        next_thought_needed: Some(true),
        confidence: Some(0.75),
        ..Default::default()
    };
    let result = handle_sequentialthinking(input, &store, &config);
    assert!(result.is_ok());
}

#[test]
fn test_max_revisions_enforced() {
    use mcp_sequential_thinking::config::AppConfig;
    use mcp_sequential_thinking::store::memory::MemoryStore;
    use mcp_sequential_thinking::tools::sequentialthinking::handle_sequentialthinking;
    use mcp_sequential_thinking::validation::rules::ThoughtInput;

    let store = MemoryStore::new(3600, true);
    let mut config = AppConfig::default();
    config.budgets.max_revisions_per_session = 1;

    // Create initial thought
    let t1 = ThoughtInput {
        session_id: Some("s1".into()),
        kind: Some("assumption".into()),
        content: Some("initial".into()),
        thought_number: Some(1),
        total_thoughts: Some(5),
        next_thought_needed: Some(true),
        ..Default::default()
    };
    handle_sequentialthinking(t1, &store, &config).unwrap();

    // First revision — OK
    let t2 = ThoughtInput {
        session_id: Some("s1".into()),
        kind: Some("revision".into()),
        content: Some("revised".into()),
        thought_number: Some(2),
        total_thoughts: Some(5),
        next_thought_needed: Some(true),
        revises_thought: Some(1),
        ..Default::default()
    };
    let r2 = handle_sequentialthinking(t2, &store, &config);
    assert!(r2.is_ok());

    // Second revision — should fail
    let t3 = ThoughtInput {
        session_id: Some("s1".into()),
        kind: Some("revision".into()),
        content: Some("revised again".into()),
        thought_number: Some(3),
        total_thoughts: Some(5),
        next_thought_needed: Some(true),
        revises_thought: Some(1),
        ..Default::default()
    };
    let r3 = handle_sequentialthinking(t3, &store, &config);
    assert!(r3.is_err());
}

#[test]
fn test_status_override() {
    use mcp_sequential_thinking::config::AppConfig;
    use mcp_sequential_thinking::store::memory::MemoryStore;
    use mcp_sequential_thinking::store::ThinkingStore;
    use mcp_sequential_thinking::tools::sequentialthinking::handle_sequentialthinking;
    use mcp_sequential_thinking::validation::rules::ThoughtInput;

    let store = MemoryStore::new(3600, true);
    let config = AppConfig::default();

    let input = ThoughtInput {
        session_id: Some("s1".into()),
        kind: Some("assumption".into()),
        content: Some("explicit status".into()),
        thought_number: Some(1),
        total_thoughts: Some(3),
        next_thought_needed: Some(true),
        status: Some("verified".into()),
        ..Default::default()
    };
    handle_sequentialthinking(input, &store, &config).unwrap();

    let session = store.get_session("s1").unwrap().unwrap();
    // Assumption normally defaults to "unverified", but we explicitly set "verified"
    assert_eq!(
        session.thoughts[0].status,
        mcp_sequential_thinking::model::thought::ThoughtStatus::Verified
    );
}
