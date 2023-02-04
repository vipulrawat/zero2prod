//! main.rs

use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configurations;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Logging
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // Panic if we can't read configuration
    let configuration = get_configurations().expect("Failed to read configurations.");

    let connection_pool = PgPool::connect_lazy(&configuration.database.connection_string().expose_secret())
        .expect("Failed to connect to Postgres");

    let address = format!("{}:{}", configuration.application.host, configuration.application.port);

    // what does this ? do
    let listener = TcpListener::bind(address)?;

    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    run(listener, connection_pool)?.await
}
