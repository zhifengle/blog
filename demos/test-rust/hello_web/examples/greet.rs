use actix_web::{dev::Server, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run()?.await
}
pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
        // .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8080")?
    .run();
    // No .await here!
    Ok(server)
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("foo");
    format!("Hoho {}!", name)
}
async fn health_check(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
    // HttpResponse::Ok()
}
async fn index(req: HttpRequest) -> &'static str {
    println!("REQ: {req:?}");
    "Hello xxx!"
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spawn_app() {
        let server = run().expect("Failed");
        let _ = tokio::spawn(server);
    }

    #[tokio::test]
    async fn health_check_works() {
        spawn_app();
        let client = reqwest::Client::new();
        let response = client
            .get("http://127.0.0.1:8080/health_check")
            .send()
            .await
            .expect("Failed to execute request.");
        // Assert
        assert!(response.status().is_success());
        assert_eq!(Some(0), response.content_length());
    }
}
