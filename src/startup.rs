//! src/startup.rs

use actix_web::{dev::Server, HttpServer, App, web};
use std::net::TcpListener;
use crate::routes::{subscribe, greet, health_check};
use sqlx::PgPool;

pub fn run(
        listener: TcpListener,
        db_pool: PgPool
) -> Result<Server, std::io::Error> {
    // Wrap the connectio in a smart pointer (ARC)
    let db_pool = web::Data::new(db_pool);

    let server = HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            // register application state (app data)
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}