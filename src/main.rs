//! main.rs

use std::net::TcpListener;
use zero2prod::startup::run;
use zero2prod::configuration::get_configurations;
use sqlx::PgPool;
use env_logger::Env;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Logging
    // Log info-level or above
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Panic if we can't read configuration
    let configuration = get_configurations().expect("Failed to read configurations.");

//    let connection = PgConnection::connect(&configuration.database.connection_string())
//            .await
//            .expect("Failed to connect to Postgres");

    let connection_pool = PgPool::connect(&configuration.database.connection_string())
            .await
            .expect("Failed to connect to Postgres");

    let address = format!("127.0.0.1:{}", configuration.application_port);

    // what does this ? do
    let listener = TcpListener::bind(address)?;

    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    run(listener, connection_pool)?.await
}
