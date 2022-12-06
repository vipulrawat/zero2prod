//! src/routes/health_check.rs

use actix_web::{web,HttpResponse};
use sqlx::PgPool;
use chrono::Utc;
use uuid::Uuid;
use tracing::Instrument;

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
    let request_span = tracing::info_span!(
            // spans, like logs, have an associated level
            // `info_span` creates a span at the info-level
            "Adding new subscriber",
            %request_id, // this is something new syntax no interpolation? What the fuck
            subscriber_email = %form.email, // tracing provides key_value pair
            subscriber_name = %form.name
    );

    // something is really wrong here, span is bad on Futures
    let _request_span_guard = request_span.enter();

    let query_span = tracing::info_span!(
            "Saving new subscriber details in the database"
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
    .instrument(query_span)
    .await
    {
        Ok(_) => {

            HttpResponse::Ok().finish()
        }
        Err(e) => {
            // used {:?} std::fmt::Debug insteas of {} std::fmt::Display
            // Debug gives the raw error useful for debugging
            // Display is more suitable for showing error to user
            tracing::error!(
                    "Failed to execute query: {:?}",
                    e
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}
