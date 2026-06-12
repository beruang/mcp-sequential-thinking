use clap::Parser;
use mcp_sequential_thinking::config::{AppConfig, Cli};

#[test]
fn test_default_config() {
    let config = AppConfig::default();
    assert_eq!(config.server.name, "mcp-sequential-thinking");
    assert_eq!(config.server.transport, "stdio");
    assert_eq!(config.budgets.max_thoughts_per_session, 32);
    assert_eq!(config.budgets.max_branches_per_session, 8);
    assert_eq!(config.budgets.max_revisions_per_session, 12);
    assert_eq!(config.budgets.max_content_chars, 4000);
    assert_eq!(config.budgets.max_reason_summary_chars, 1000);
    assert_eq!(config.budgets.max_evidence_items_per_thought, 10);
    assert_eq!(config.budgets.max_tags_per_thought, 20);
    assert_eq!(config.retention.mode, "ephemeral");
    assert_eq!(config.retention.ttl_seconds, 3600);
    assert!(!config.retention.persist);
    assert!(config.logging.enabled);
    assert!(!config.logging.log_raw_content);
    assert!(config.redaction.enabled);
    assert!(config.redaction.redact_before_store);
    assert!(config.redaction.redact_before_log);
    assert!(config.compatibility.accept_legacy_thought_field);
    assert_eq!(config.compatibility.default_legacy_kind, "observation");
    assert_eq!(config.compatibility.default_session_id, "default");
}

#[test]
fn test_cli_flag_disable_logging() {
    let cli = Cli::try_parse_from(["mcp-sequential-thinking", "--disable-logging"]).unwrap();
    assert!(cli.disable_logging);
}

#[test]
fn test_cli_flag_disable_redaction() {
    let cli = Cli::try_parse_from(["mcp-sequential-thinking", "--disable-redaction"]).unwrap();
    assert!(cli.disable_redaction);
}

#[test]
fn test_cli_flag_compatibility_mode() {
    let cli = Cli::try_parse_from(["mcp-sequential-thinking", "--compatibility-mode"]).unwrap();
    assert!(cli.compatibility_mode);
}

#[test]
fn test_cli_flag_max_thoughts() {
    let cli = Cli::try_parse_from(["mcp-sequential-thinking", "--max-thoughts", "64"]).unwrap();
    assert_eq!(cli.max_thoughts, Some(64));
}

#[test]
fn test_cli_flag_ttl_seconds() {
    let cli = Cli::try_parse_from(["mcp-sequential-thinking", "--ttl-seconds", "7200"]).unwrap();
    assert_eq!(cli.ttl_seconds, Some(7200));
}

#[test]
fn test_cli_flag_log_level() {
    let cli = Cli::try_parse_from(["mcp-sequential-thinking", "--log-level", "debug"]).unwrap();
    assert_eq!(cli.log_level, "debug");
}

#[test]
fn test_cli_overrides_config() {
    let cli = Cli::try_parse_from([
        "mcp-sequential-thinking",
        "--max-thoughts",
        "64",
        "--ttl-seconds",
        "7200",
        "--disable-logging",
        "--disable-redaction",
    ])
    .unwrap();
    let config = AppConfig::load(&cli).unwrap();
    assert_eq!(config.budgets.max_thoughts_per_session, 64);
    assert_eq!(config.retention.ttl_seconds, 7200);
    assert!(!config.logging.enabled);
    assert!(!config.redaction.enabled);
}
