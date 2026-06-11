use clap::Parser;

use mcp_sequential_thinking::config;
use mcp_sequential_thinking::server;

use config::Cli;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let config = config::AppConfig::load(&cli)?;

    tracing_subscriber::fmt()
        .with_max_level(match cli.log_level.as_str() {
            "trace" => tracing::Level::TRACE,
            "debug" => tracing::Level::DEBUG,
            "warn" => tracing::Level::WARN,
            "error" => tracing::Level::ERROR,
            _ => tracing::Level::INFO,
        })
        .with_writer(std::io::stderr)
        .init();

    tracing::info!("mcp-sequential-thinking starting");

    server::run_server(config).await?;

    Ok(())
}
