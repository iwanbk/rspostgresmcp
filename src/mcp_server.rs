use rmcp::{Error as McpError, ServerHandler, model::*, schemars, tool};

use crate::db;

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct TableNameRequest {
    #[schemars(description = "name of the table to get schema for")]
    pub name: String,
}

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

    #[tool(description = "Get schema of a specific table")]
    async fn get_schema(
        &self,
        #[tool(aggr)] TableNameRequest { name }: TableNameRequest,
    ) -> String {
        match self.db.get_table_schema(&name).await {
            Ok(schema) => match serde_json::to_string_pretty(&schema) {
                Ok(json) => json,
                Err(e) => format!("JSON serialization error: {}", e),
            },
            Err(e) => format!("Error getting schema for table {}: {}", name, e),
        }
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
