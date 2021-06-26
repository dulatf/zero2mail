use actix_web::{web, Error, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;
use web::{Data, Form};

#[derive(serde::Deserialize)]
pub struct SubscribeFormData {
    name: String,
    email: String,
}

pub async fn subscribe(
    form: Form<SubscribeFormData>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, Error> {
    sqlx::query!(
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
    .map_err(|e| {
        eprintln!("Failed to execute query {}", e);
        actix_web::error::ErrorInternalServerError(format!("{}", e))
    })?;
    //.unwrap();
    //HttpResponse::Ok().finish()
    Ok(HttpResponse::Ok().finish())
}
