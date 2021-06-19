use actix_web::{web, HttpResponse};
#[derive(serde::Deserialize)]
#[allow(dead_code)]

pub struct SubscribeFormData {
    name: String,
    email: String,
}

pub async fn subscribe(_form: web::Form<SubscribeFormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
