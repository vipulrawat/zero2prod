//! src/routes/health_check.rs

use actix_web::{web,HttpResponse};
use sqlx::PgPool;
use chrono::Utc;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String
}

pub async fn subscribe(
        form: web::Form<FormData>,
        pool: web::Data<PgPool>
) -> HttpResponse {
    let request_id = Uuid::new_v4();

    // Adding user info to logs improves the log
    // and providing good breadcrumbs for error tracing
    // BUT this falls under GDPR rules (should have some mechanism to delete these on user request)
    log::info!(
            "request_id {} - Adding '{}' '{}' as a new subscriber",
            request_id,
            form.email,
            form.name
    );

    log::info!(
            "request_id {} - Saving new subscriber details in the database",
            request_id
    );

    match sqlx::query!(
            r#"
            INSERT INTO subscriptions (id, email, name, subscribed_at)
            VALUES ($1, $2, $3, $4)
            "#,
            Uuid::new_v4(),
            form.email,
            form.name,
            Utc::now()
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => {
            log::info!(
                    "request_id {} - New subscriber details have been saved",
                    request_id
            );
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            // used {:?} std::fmt::Debug insteas of {} std::fmt::Display
            // Debug gives the raw error useful for debugging
            // Display is more suitable for showing error to user
            log::error!(
                    "request_id {} - Failed to execute query: {:?}",
                    request_id,
                    e
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}
