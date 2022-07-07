#![allow(unused_imports)]
#![allow(dead_code)]

use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

async fn greeting(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}", &name)
}

async fn health_check(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            // .route("/", web::get().to(greeting))
            // .route("/{name}", web::get().to(greeting))
            .route("/health_check", web::get().to(health_check))
    })
    .listen(listener)?
    // .bind("127.0.0.1:8000")?
    .run();

    Ok(server)
}
