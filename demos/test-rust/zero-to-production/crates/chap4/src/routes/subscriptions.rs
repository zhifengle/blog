use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}
// setx RUST_LOG log; Windows
pub async fn subscribe(form: web::Form<FormData>) -> HttpResponse {
    let request_span = tracing::info_span!(
    "Adding a new subscriber.",
    subscriber_email = %form.email,
    subscriber_name= %form.name
    );
    let _request_span_guard = request_span.enter();
    let query_span = tracing::info_span!("Saving new subscriber details in the database");
    tracing::info!("subscribe");
    HttpResponse::Ok().finish()
}
