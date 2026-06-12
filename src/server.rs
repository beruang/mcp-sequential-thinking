use std::sync::Arc;

use rmcp::handler::server::ServerHandler;
use rmcp::model::{CallToolRequestParams, CallToolResult, ListToolsResult, Tool, ToolAnnotations};
use rmcp::service::{RequestContext, RoleServer};
use rmcp::ErrorData;

use crate::config::AppConfig;
use crate::error::ThinkingError;
use crate::store::memory::MemoryStore;
use crate::tools::clear_session::{handle_clear_session, ClearSessionInput};
use crate::tools::export_session::{handle_export_session, ExportSessionInput};
use crate::tools::get_session::{handle_get_session, GetSessionInput};
use crate::tools::list_sessions::{handle_list_sessions, ListSessionsInput};
use crate::tools::sequentialthinking::handle_sequential_thinking;
use crate::tools::sequentialthinking::handle_sequentialthinking;
use crate::validation::rules::ThoughtInput;

fn to_error(err: ThinkingError) -> ErrorData {
    ErrorData::internal_error(err.to_string(), None)
}

fn schema_for<T: schemars::JsonSchema>() -> Arc<serde_json::Map<String, serde_json::Value>> {
    let schema = schemars::schema_for!(T);
    let value = serde_json::to_value(&schema).unwrap_or_default();
    match value {
        serde_json::Value::Object(map) => Arc::new(map),
        _ => Arc::new(serde_json::Map::new()),
    }
}

pub async fn run_server(config: AppConfig) -> anyhow::Result<()> {
    let retention_enabled = matches!(config.retention.mode.as_str(), "ephemeral");
    let store = Arc::new(MemoryStore::new(
        config.retention.ttl_seconds,
        retention_enabled,
    ));

    let service = SequentialThinkingService::new(store, config);

    tracing::info!("Server configured, starting stdio transport");

    let running = rmcp::serve_server(service, (tokio::io::stdin(), tokio::io::stdout())).await?;
    running.waiting().await?;

    Ok(())
}

pub struct SequentialThinkingService {
    store: Arc<MemoryStore>,
    config: AppConfig,
}

impl SequentialThinkingService {
    pub fn new(store: Arc<MemoryStore>, config: AppConfig) -> Self {
        Self { store, config }
    }
}

impl ServerHandler for SequentialThinkingService {
    fn get_info(&self) -> rmcp::model::InitializeResult {
        let capabilities = rmcp::model::ServerCapabilities::builder()
            .enable_tools()
            .build();
        rmcp::model::InitializeResult::new(capabilities)
            .with_server_info(rmcp::model::Implementation::new(
                "mcp-sequential-thinking",
                env!("CARGO_PKG_VERSION"),
            ))
            .with_instructions(
                "Structured reasoning trace server. Use sequentialthinking to record thoughts.",
            )
    }

    fn get_tool(&self, name: &str) -> Option<Tool> {
        self.all_tools().tools.into_iter().find(|t| t.name == name)
    }

    fn list_tools(
        &self,
        _params: Option<rmcp::model::PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> impl std::future::Future<Output = Result<ListToolsResult, ErrorData>> + Send + '_ {
        std::future::ready(Ok(self.all_tools()))
    }

    fn call_tool(
        &self,
        params: CallToolRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> impl std::future::Future<Output = Result<CallToolResult, ErrorData>> + Send + '_ {
        let result = self.route_tool_call(&params.name, &params.arguments);
        std::future::ready(result)
    }
}

impl SequentialThinkingService {
    fn all_tools(&self) -> ListToolsResult {
        let tools = vec![
            Tool::new(
                "sequentialthinking",
                "Record one structured thought in a reasoning session.",
                schema_for::<ThoughtInput>(),
            )
            .with_annotations(
                ToolAnnotations::new()
                    .read_only(false)
                    .destructive(false)
                    .idempotent(false)
                    .open_world(false),
            ),
            Tool::new(
                "sequential_thinking",
                "Alias for sequentialthinking.",
                schema_for::<ThoughtInput>(),
            )
            .with_annotations(
                ToolAnnotations::new()
                    .read_only(false)
                    .destructive(false)
                    .idempotent(false)
                    .open_world(false),
            ),
            Tool::new(
                "get_thinking_session",
                "Return one thinking session by ID.",
                schema_for::<GetSessionInput>(),
            )
            .with_annotations(
                ToolAnnotations::new()
                    .read_only(true)
                    .destructive(false)
                    .idempotent(true)
                    .open_world(false),
            ),
            Tool::new(
                "list_thinking_sessions",
                "List active thinking sessions.",
                schema_for::<ListSessionsInput>(),
            )
            .with_annotations(
                ToolAnnotations::new()
                    .read_only(true)
                    .destructive(false)
                    .idempotent(true)
                    .open_world(false),
            ),
            Tool::new(
                "clear_thinking_session",
                "Remove one session from memory. Idempotent.",
                schema_for::<ClearSessionInput>(),
            )
            .with_annotations(
                ToolAnnotations::new()
                    .read_only(false)
                    .destructive(true)
                    .idempotent(true)
                    .open_world(false),
            ),
            Tool::new(
                "export_thinking_session",
                "Export one session trace in JSON, JSONL, or Markdown.",
                schema_for::<ExportSessionInput>(),
            )
            .with_annotations(
                ToolAnnotations::new()
                    .read_only(true)
                    .destructive(false)
                    .idempotent(true)
                    .open_world(false),
            ),
        ];
        ListToolsResult {
            tools,
            next_cursor: None,
            ..Default::default()
        }
    }

    fn route_tool_call(
        &self,
        name: &str,
        arguments: &Option<serde_json::Map<String, serde_json::Value>>,
    ) -> Result<CallToolResult, ErrorData> {
        let args = serde_json::Value::Object(arguments.clone().unwrap_or_default());

        match name {
            "sequentialthinking" => {
                let input: ThoughtInput = serde_json::from_value(args)
                    .map_err(|e| ErrorData::invalid_params(e.to_string(), None))?;
                let output = handle_sequentialthinking(input, &*self.store, &self.config)
                    .map_err(to_error)?;
                Ok(CallToolResult::structured(
                    serde_json::to_value(&output)
                        .map_err(|e| ErrorData::internal_error(e.to_string(), None))?,
                ))
            }
            "sequential_thinking" => {
                let input: ThoughtInput = serde_json::from_value(args)
                    .map_err(|e| ErrorData::invalid_params(e.to_string(), None))?;
                let output = handle_sequential_thinking(input, &*self.store, &self.config)
                    .map_err(to_error)?;
                Ok(CallToolResult::structured(
                    serde_json::to_value(&output)
                        .map_err(|e| ErrorData::internal_error(e.to_string(), None))?,
                ))
            }
            "get_thinking_session" => {
                let input: GetSessionInput = serde_json::from_value(args)
                    .map_err(|e| ErrorData::invalid_params(e.to_string(), None))?;
                let output = handle_get_session(input, &*self.store).map_err(to_error)?;
                Ok(CallToolResult::structured(
                    serde_json::to_value(&output)
                        .map_err(|e| ErrorData::internal_error(e.to_string(), None))?,
                ))
            }
            "list_thinking_sessions" => {
                let input: ListSessionsInput = serde_json::from_value(args)
                    .map_err(|e| ErrorData::invalid_params(e.to_string(), None))?;
                let output = handle_list_sessions(input, &*self.store).map_err(to_error)?;
                Ok(CallToolResult::structured(
                    serde_json::to_value(&output)
                        .map_err(|e| ErrorData::internal_error(e.to_string(), None))?,
                ))
            }
            "clear_thinking_session" => {
                let input: ClearSessionInput = serde_json::from_value(args)
                    .map_err(|e| ErrorData::invalid_params(e.to_string(), None))?;
                let output = handle_clear_session(input, &*self.store).map_err(to_error)?;
                Ok(CallToolResult::structured(
                    serde_json::to_value(&output)
                        .map_err(|e| ErrorData::internal_error(e.to_string(), None))?,
                ))
            }
            "export_thinking_session" => {
                let input: ExportSessionInput = serde_json::from_value(args)
                    .map_err(|e| ErrorData::invalid_params(e.to_string(), None))?;
                let output = handle_export_session(input, &*self.store).map_err(to_error)?;
                Ok(CallToolResult::structured(
                    serde_json::to_value(&output)
                        .map_err(|e| ErrorData::internal_error(e.to_string(), None))?,
                ))
            }
            _ => Err(ErrorData::method_not_found::<
                rmcp::model::CallToolRequestMethod,
            >()),
        }
    }
}
