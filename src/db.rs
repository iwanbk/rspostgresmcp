use sqlx::{PgPool, Row, postgres::PgPoolOptions};
use std::sync::Arc;

#[derive(Clone)]
pub struct DB {
    pool: Arc<PgPool>,
}

impl DB {
    pub async fn new(dsn: String) -> anyhow::Result<Self> {
        // Create a connection pool
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&dsn)
            .await?;

        Ok(Self {
            pool: Arc::new(pool),
        })
    }
    pub async fn get_table_names(&self) -> anyhow::Result<Vec<String>> {
        // Query to get all table names from the public schema
        let rows = sqlx::query(
            "SELECT table_name FROM information_schema.tables WHERE table_schema = 'public'",
        )
        .fetch_all(&*self.pool)
        .await?;

        // Map the rows to a vector of strings
        let table_names = rows
            .iter()
            .map(|row| row.get::<String, _>("table_name"))
            .collect();

        Ok(table_names)
    }
}
