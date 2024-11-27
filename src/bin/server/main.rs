use tide::config::Config;
use tide::domain::booking;
use tide::inbound::http::{HttpConfig, HttpServer};
use tide::outbound::postgres::{PgConfig, Postgres};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let config = Config::from_env()?;

    // Initialize outbound adapters needed by core services
    let pg_config = PgConfig { url: &config.db_url };
    let postgres = Postgres::from_config(pg_config).await?;

    // Initialize core services
    let bookings = booking::service::Service::new(postgres.clone());

    // Initialize inbound adapters to consume core services
    let server_config = HttpConfig { port: "8080" };
    let server = HttpServer::new(server_config, bookings).await?;
    server.serve().await?;

    Ok(())
}