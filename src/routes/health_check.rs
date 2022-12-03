//! src/routes/health_check.rs
use actix_web::{HttpResponse, HttpRequest};

pub async fn health_check(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().finish()
}
