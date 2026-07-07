use std::time::Duration;

use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

use crate::domain::error::ServerError;

#[derive(Clone)]
pub struct PostgresDatabase {
    pool: Pool<Postgres>,
}

impl PostgresDatabase {
    pub async fn connect(connection_string: &str) -> Result<Self, ServerError> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .idle_timeout(Duration::from_secs(30))
            .connect(connection_string)
            .await?;

        Ok(Self { pool })
    }

    pub async fn run_migrations(&self) -> Result<(), ServerError> {
        sqlx::migrate!("./migrations")
            .run(&self.pool)
            .await
            .map_err(|e| ServerError::Database(e.into()))?;

        Ok(())
    }

    pub fn pool(&self) -> &Pool<Postgres> {
        &self.pool
    }
}
