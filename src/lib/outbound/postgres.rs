//! Module [postgres] is an outbound adapter for a PostgreSQL relational database.

mod booking_repository;

use anyhow::Context;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

/// [PgConfig] contains the database credentials and other options needed to instantiate [Postgres].
pub struct PgConfig<'cfg> {
    pub url: &'cfg str,
}

#[derive(Clone, Debug)]
pub struct Postgres {
    pool: PgPool,
}

impl Postgres {
    /// Creates a new instance of [Postgres] from a [PgConfig].
    ///
    /// `from_config` initializes a connection pool with a sensible number of max connections,
    /// then runs any pending migrations.
    ///
    /// If the connection and migrations are successful, returns an [Ok] containing the
    /// Postgres instance.
    ///
    /// If connection or migration fails, the database is unavailable or
    /// the provided configuration is malformed, so the offending errors are surfaced out for
    /// reporting.
    pub async fn from_config(config: PgConfig<'_>) -> anyhow::Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&config.url)
            .await
            .context("failed to connect to DB")?;

        sqlx::migrate!()
            .run(&pool)
            .await
            .context("failed to run DB migrations")?;

        Ok(Self { pool })
    }
}
