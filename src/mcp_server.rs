use rmcp::{Error as McpError, ServerHandler, model::*, tool};

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

    #[tool(description = "List tables in a schema")]
    async fn list_tables(&self) -> Result<CallToolResult, McpError> {
        let table_names = self
            .db
            .get_table_names()
            .await
            .map_err(|e| McpError::internal_error(format!("Database error: {}", e), None))?;

        let json = serde_json::to_string_pretty(&table_names).map_err(|e| {
            McpError::internal_error(format!("JSON serialization error: {}", e), None)
        })?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
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
