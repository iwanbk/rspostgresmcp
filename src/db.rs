use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row, postgres::PgPoolOptions};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct TableColumn {
    pub name: String,
    pub data_type: String,
    pub max_length: Option<i32>,
    pub is_nullable: bool,
    pub default_value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableIndex {
    pub name: String,
    pub columns: Vec<String>,
    pub is_unique: bool,
    pub is_primary: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableSchema {
    pub columns: Vec<TableColumn>,
    pub indexes: Vec<TableIndex>,
}

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

    pub async fn get_table_schema(&self, table_name: &str) -> anyhow::Result<TableSchema> {
        // Query to get column information for the specified table
        let rows = sqlx::query(
            r#"
            SELECT 
                column_name, 
                data_type, 
                character_maximum_length,
                is_nullable,
                column_default
            FROM 
                information_schema.columns 
            WHERE 
                table_schema = 'public' 
                AND table_name = $1
            ORDER BY 
                ordinal_position
            "#,
        )
        .bind(table_name)
        .fetch_all(&*self.pool)
        .await?;

        // Map the rows to a vector of TableColumn structs
        let columns = rows
            .iter()
            .map(|row| TableColumn {
                name: row.get("column_name"),
                data_type: row.get("data_type"),
                max_length: row.get("character_maximum_length"),
                is_nullable: row.get::<String, _>("is_nullable") == "YES",
                default_value: row.get("column_default"),
            })
            .collect();

        // Query to get index information for the specified table
        let index_rows = sqlx::query(
            r#"
            SELECT
                i.relname as index_name,
                a.attname as column_name,
                ix.indisunique as is_unique,
                ix.indisprimary as is_primary
            FROM
                pg_class t,
                pg_class i,
                pg_index ix,
                pg_attribute a
            WHERE
                t.oid = ix.indrelid
                AND i.oid = ix.indexrelid
                AND a.attrelid = t.oid
                AND a.attnum = ANY(ix.indkey)
                AND t.relkind = 'r'
                AND t.relname = $1
            ORDER BY
                i.relname, a.attnum
            "#,
        )
        .bind(table_name)
        .fetch_all(&*self.pool)
        .await?;

        // Process index information into a map of index name -> index details
        let mut index_map = std::collections::HashMap::new();
        for row in index_rows.iter() {
            let index_name: String = row.get("index_name");
            let column_name: String = row.get("column_name");
            let is_unique: bool = row.get("is_unique");
            let is_primary: bool = row.get("is_primary");

            index_map
                .entry(index_name.clone())
                .or_insert_with(|| TableIndex {
                    name: index_name,
                    columns: Vec::new(),
                    is_unique,
                    is_primary,
                })
                .columns
                .push(column_name);
        }

        // Convert the map to a vector of TableIndex
        let indexes = index_map.into_values().collect();

        Ok(TableSchema { columns, indexes })
    }
}
