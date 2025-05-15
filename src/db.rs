use sqlx::{PgPool, postgres::PgPoolOptions};
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
}
