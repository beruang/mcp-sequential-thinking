use clap::Parser;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AppConfig {
    #[serde(default)]
    pub server: ServerConfig,
    #[serde(default)]
    pub compatibility: CompatibilityConfig,
    #[serde(default)]
    pub budgets: BudgetConfig,
    #[serde(default)]
    pub retention: RetentionConfig,
    #[serde(default)]
    pub logging: LoggingConfig,
    #[serde(default)]
    pub redaction: RedactionConfig,
    #[serde(default)]
    pub behavior: BehaviorConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerConfig {
    #[serde(default = "default_server_name")]
    pub name: String,
    #[serde(default = "default_transport")]
    pub transport: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompatibilityConfig {
    #[serde(default = "default_true")]
    pub accept_legacy_thought_field: bool,
    #[serde(default = "default_legacy_kind")]
    pub default_legacy_kind: String,
    #[serde(default = "default_session_id")]
    pub default_session_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BudgetConfig {
    #[serde(default = "default_max_thoughts")]
    pub max_thoughts_per_session: u32,
    #[serde(default = "default_max_branches")]
    pub max_branches_per_session: u32,
    #[serde(default = "default_max_revisions")]
    pub max_revisions_per_session: u32,
    #[serde(default = "default_max_content_chars")]
    pub max_content_chars: usize,
    #[serde(default = "default_max_reason_summary_chars")]
    pub max_reason_summary_chars: usize,
    #[serde(default = "default_max_evidence_items")]
    pub max_evidence_items_per_thought: usize,
    #[serde(default = "default_max_tags")]
    pub max_tags_per_thought: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RetentionConfig {
    #[serde(default = "default_retention_mode")]
    pub mode: String,
    #[serde(default = "default_ttl")]
    pub ttl_seconds: u64,
    #[serde(default)]
    pub persist: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoggingConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default)]
    pub log_raw_content: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RedactionConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_true")]
    pub redact_before_store: bool,
    #[serde(default = "default_true")]
    pub redact_before_log: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BehaviorConfig {
    #[serde(default = "default_true")]
    pub auto_supersede_revised_thoughts: bool,
    #[serde(default = "default_true")]
    pub allow_non_contiguous_thought_numbers: bool,
    #[serde(default = "default_true")]
    pub allow_final_decision_after_budget_exceeded: bool,
}

#[derive(Parser)]
#[command(name = "mcp-sequential-thinking")]
pub struct Cli {
    #[arg(long)]
    pub config: Option<PathBuf>,

    #[arg(long)]
    pub max_thoughts: Option<u32>,

    #[arg(long)]
    pub ttl_seconds: Option<u64>,

    #[arg(long)]
    pub disable_logging: bool,

    #[arg(long)]
    pub disable_redaction: bool,

    #[arg(long)]
    pub compatibility_mode: bool,

    #[arg(long, default_value = "info")]
    pub log_level: String,
}

// Default value helpers
fn default_server_name() -> String {
    "mcp-sequential-thinking".to_string()
}
fn default_transport() -> String {
    "stdio".to_string()
}
fn default_true() -> bool {
    true
}
fn default_legacy_kind() -> String {
    "observation".to_string()
}
fn default_session_id() -> String {
    "default".to_string()
}
fn default_max_thoughts() -> u32 {
    32
}
fn default_max_branches() -> u32 {
    8
}
fn default_max_revisions() -> u32 {
    12
}
fn default_max_content_chars() -> usize {
    4000
}
fn default_max_reason_summary_chars() -> usize {
    1000
}
fn default_max_evidence_items() -> usize {
    10
}
fn default_max_tags() -> usize {
    20
}
fn default_retention_mode() -> String {
    "ephemeral".to_string()
}
fn default_ttl() -> u64 {
    3600
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            name: default_server_name(),
            transport: default_transport(),
        }
    }
}
impl Default for CompatibilityConfig {
    fn default() -> Self {
        Self {
            accept_legacy_thought_field: true,
            default_legacy_kind: default_legacy_kind(),
            default_session_id: default_session_id(),
        }
    }
}
impl Default for BudgetConfig {
    fn default() -> Self {
        Self {
            max_thoughts_per_session: default_max_thoughts(),
            max_branches_per_session: default_max_branches(),
            max_revisions_per_session: default_max_revisions(),
            max_content_chars: default_max_content_chars(),
            max_reason_summary_chars: default_max_reason_summary_chars(),
            max_evidence_items_per_thought: default_max_evidence_items(),
            max_tags_per_thought: default_max_tags(),
        }
    }
}
impl Default for RetentionConfig {
    fn default() -> Self {
        Self {
            mode: default_retention_mode(),
            ttl_seconds: default_ttl(),
            persist: false,
        }
    }
}
impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            log_raw_content: false,
        }
    }
}
impl Default for RedactionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            redact_before_store: true,
            redact_before_log: true,
        }
    }
}
impl Default for BehaviorConfig {
    fn default() -> Self {
        Self {
            auto_supersede_revised_thoughts: true,
            allow_non_contiguous_thought_numbers: true,
            allow_final_decision_after_budget_exceeded: true,
        }
    }
}

impl AppConfig {
    pub fn load(cli: &Cli) -> anyhow::Result<Self> {
        let mut config = AppConfig::default();

        // Load from config file
        let config_path = cli.config.clone().or_else(|| {
            std::env::var("SEQUENTIAL_THINKING_CONFIG")
                .ok()
                .map(PathBuf::from)
        });

        if let Some(path) = config_path {
            if path.exists() {
                let content = std::fs::read_to_string(&path)?;
                config = serde_json::from_str(&content)?;
            }
        }

        // Environment variable overrides
        if let Ok(val) = std::env::var("SEQUENTIAL_THINKING_TTL_SECONDS") {
            if let Ok(n) = val.parse() {
                config.retention.ttl_seconds = n;
            }
        }
        if let Ok(val) = std::env::var("SEQUENTIAL_THINKING_MAX_THOUGHTS") {
            if let Ok(n) = val.parse() {
                config.budgets.max_thoughts_per_session = n;
            }
        }
        if let Ok(val) = std::env::var("SEQUENTIAL_THINKING_REDACTION") {
            config.redaction.enabled = val.to_lowercase() == "true";
        }
        if std::env::var("DISABLE_THOUGHT_LOGGING").is_ok() {
            config.logging.enabled = false;
        }

        // CLI overrides
        if let Some(n) = cli.max_thoughts {
            config.budgets.max_thoughts_per_session = n;
        }
        if let Some(n) = cli.ttl_seconds {
            config.retention.ttl_seconds = n;
        }
        if cli.disable_logging {
            config.logging.enabled = false;
        }
        if cli.disable_redaction {
            config.redaction.enabled = false;
        }
        if cli.compatibility_mode {
            config.compatibility.accept_legacy_thought_field = true;
        }

        Ok(config)
    }
}
