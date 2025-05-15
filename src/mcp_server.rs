use rmcp::{ServerHandler, model::*, tool};

use crate::db;

#[derive(Clone)]
pub struct McpServer {
    pub db: db::DB,
}

#[tool(tool_box)]
impl McpServer {
    pub fn new(db: db::DB) -> Self {
        Self { db }
    }
}

#[tool(tool_box)]
impl ServerHandler for McpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("Postgres MCP server".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}
