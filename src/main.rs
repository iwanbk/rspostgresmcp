use clap::Parser;
use rmcp::{ServerHandler, model::*, tool, transport::sse_server::SseServer};

#[derive(Parser, Debug)]
#[clap(name = "rspostgresmcp", about = "Postgres MCP server")]
struct Cli {
    /// postgres connection string
    #[clap(
        long,
        default_value = "postgres://postgres:password@localhost:5432/postgres"
    )]
    dsn: String,

    /// address to bind to
    #[clap(long, default_value = "127.0.0.1:9000")]
    addr: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let ct = SseServer::serve(cli.addr.parse()?)
        .await?
        .with_service(MCP::new);

    tokio::signal::ctrl_c().await?;
    ct.cancel();
    Ok(())
}
#[derive(Clone)]
struct MCP {}

#[tool(tool_box)]
impl MCP {
    fn new() -> Self {
        Self {}
    }
}

#[tool(tool_box)]
impl ServerHandler for MCP {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("Postgres MCP server".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}
