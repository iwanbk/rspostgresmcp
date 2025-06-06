use anyhow::Result;
use rspostgresmcp::{db::DB, mcp_server::McpServer};
use sqlx::PgPool;
use std::path::PathBuf;
use std::time::Duration;
use testcontainers::{Container, RunnableImage, clients, images::postgres::Postgres};
use tokio::time::sleep;

// Path to the pagila.sql fixture
fn get_pagila_sql_path() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/fixtures/pagila.sql");
    path
}

struct PostgresContainer<'a> {
    _container: Container<'a, Postgres>,
    port: u16,
}

impl<'a> PostgresContainer<'a> {
    fn new(docker: &'a clients::Cli) -> Self {
        let pagila_sql_path = get_pagila_sql_path();

        // Convert path to string
        let pagila_path_str = pagila_sql_path.to_str().expect("Invalid path");

        // Configure PostgreSQL container with Pagila SQL mounted
        let postgres_image = RunnableImage::from(Postgres::default())
            .with_volume((
                pagila_path_str.to_string(),
                "/docker-entrypoint-initdb.d/pagila.sql".to_string(),
            ))
            .with_env_var(("POSTGRES_PASSWORD".to_string(), "postgres".to_string()))
            .with_env_var(("POSTGRES_DB".to_string(), "pagila".to_string()));

        // Start the container
        let container = docker.run(postgres_image);
        let port = container.get_host_port_ipv4(5432);

        Self {
            _container: container,
            port,
        }
    }

    fn get_connection_string(&self) -> String {
        format!(
            "postgres://postgres:postgres@localhost:{}/pagila",
            self.port
        )
    }

    // Wait for PostgreSQL to be ready by attempting to connect
    async fn wait_until_ready(&self) -> Result<()> {
        let dsn = self.get_connection_string();
        let max_attempts = 10;
        let retry_delay = Duration::from_secs(1);

        for attempt in 1..=max_attempts {
            match PgPool::connect(&dsn).await {
                Ok(_) => {
                    println!("PostgreSQL is ready after {} attempts", attempt);
                    return Ok(());
                }
                Err(e) => {
                    if attempt == max_attempts {
                        return Err(anyhow::anyhow!(
                            "Failed to connect to PostgreSQL after {} attempts: {}",
                            max_attempts,
                            e
                        ));
                    }
                    println!(
                        "Waiting for PostgreSQL to be ready (attempt {}/{}): {}",
                        attempt, max_attempts, e
                    );
                    sleep(retry_delay).await;
                }
            }
        }

        unreachable!()
    }
}

#[tokio::test]
async fn test_list_tables() -> Result<()> {
    // Start PostgreSQL container with Pagila data
    let docker = clients::Cli::default();
    let postgres = PostgresContainer::new(&docker);

    // Wait for PostgreSQL to be ready by polling connection
    postgres.wait_until_ready().await?;
    println!("PostgreSQL is ready with Pagila data loaded");

    // Create DB connection using the container's port
    let dsn = postgres.get_connection_string();
    let db = DB::new(dsn).await?;

    // Create MCP server
    let mcp_server = McpServer::new(db);

    // Instead of using MCP client/server, we'll directly test the DB functionality
    // since we can't easily create an MCP client in the test

    // Get the table names directly from the database
    let table_names = mcp_server.db.get_table_names().await?;

    // Verify that we have the expected Pagila tables
    assert!(table_names.contains(&"actor".to_string()));
    assert!(table_names.contains(&"film".to_string()));
    assert!(table_names.contains(&"customer".to_string()));
    assert!(table_names.contains(&"rental".to_string()));

    println!("Found {} tables: {:?}", table_names.len(), table_names);

    Ok(())
}
