use clap::Parser;
use rmcp::transport::sse_server::SseServer;

mod db;
mod mcp_server;

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

    // Initialize DB connection
    let db = db::DB::new(cli.dsn.clone()).await?;

    // Create an MCP instance
    let mcp = mcp_server::McpServer::new(db);

    let ct = SseServer::serve(cli.addr.parse()?)
        .await?
        .with_service(move || mcp.clone());

    tokio::signal::ctrl_c().await?;
    ct.cancel();
    Ok(())
}
