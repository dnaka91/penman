use anyhow::Result;
use sqlx::postgres::{PgPool, PgPoolOptions};

pub struct PostgresProvider {
    pool: PgPool,
}

impl PostgresProvider {
    async fn new(database_url: String) -> Result<Self> {
        let pg_pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;

        Ok(PostgresProvider { pool: pg_pool })
    }
}

