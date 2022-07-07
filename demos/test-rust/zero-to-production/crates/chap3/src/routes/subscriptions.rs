use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}
pub async fn subscribe(_web: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
