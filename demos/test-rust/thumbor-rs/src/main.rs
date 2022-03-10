mod pb;

// https://github.com/tyrchen/geektime-rust/tree/master/05_thumbor
// 接口设计: resize watermark clop
// 多个操作的顺序?

use actix_web::{
    error::{self, ErrorBadRequest},
    get, web, App, Error, HttpResponse, HttpServer, Responder,
};
use pb::*;
use percent_encoding::percent_decode_str;
use serde::Deserialize;

#[derive(Deserialize)]
struct Params {
    spec: String,
    url: String,
}
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    HttpServer::new(|| {
        App::new()
            // .wrap(middleware::Logger::default())
            .service(generate)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;
    Ok(())
}

// 解析路径
// 没有使用宏
// .service(web::resource("/manual").route(web::post().to(index_manual)))
// 可以直接返回 impl Responder: String
#[get("/image/{spec}/{url}")]
async fn generate(params: web::Path<Params>) -> Result<HttpResponse, Error> {
    let url = percent_decode_str(&params.url).decode_utf8_lossy();
    let spec: ImageSpec = params
        .spec
        .as_str()
        .try_into()
        .map_err(|_| ErrorBadRequest("invalid spec"))?;

    Ok(HttpResponse::Ok().body(format!("url: {}\n spec: {:#?}", url, spec)))
}
