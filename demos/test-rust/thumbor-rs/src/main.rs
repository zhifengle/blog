mod engine;
mod pb;

// https://github.com/tyrchen/geektime-rust/tree/master/05_thumbor
// 接口设计: resize watermark clop
// 多个操作的顺序?

use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    sync::{Arc, Mutex},
};

use actix_web::{
    error::{ErrorBadRequest, ErrorInternalServerError},
    get, web, App, Error, HttpResponse, HttpServer,
};
use bytes::Bytes;
use image::ImageOutputFormat;
use lru::LruCache;
use percent_encoding::{percent_decode_str, percent_encode, NON_ALPHANUMERIC};
use serde::Deserialize;
use tracing::{info, instrument};

use crate::engine::Engine;
use engine::Photon;
use pb::*;

#[derive(Deserialize)]
struct Params {
    spec: String,
    url: String,
}

// cache 别名
type Cache = Arc<Mutex<LruCache<u64, Bytes>>>;

// This struct represents state
struct AppState {
    app_cache: Cache,
}
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    // https://actix.rs/docs/application/
    // Shared Mutable State
    let state = web::Data::new(AppState {
        app_cache: Arc::new(Mutex::new(LruCache::new(1024))),
    });

    // https://images.pexels.com/photos/2470905/pexels-photo-2470905.jpeg?auto=compress&cs=tinysrgb&dpr=2&h=750&w=1260
    print_test_url("https://images.pexels.com/photos/1562477/pexels-photo-1562477.jpeg?auto=compress&cs=tinysrgb&dpr=3&h=750&w=1260");
    HttpServer::new(move || {
        App::new()
            // .wrap(middleware::Logger::default())
            .app_data(state.clone())
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
async fn generate(
    params: web::Path<Params>,
    data: web::Data<AppState>,
) -> anyhow::Result<HttpResponse, Error> {
    let url = percent_decode_str(&params.url).decode_utf8_lossy();
    let spec: ImageSpec = params
        .spec
        .as_str()
        .try_into()
        .map_err(|_| ErrorBadRequest("invalid spec"))?;

    let data = retrieve_image(&url, data.app_cache.clone())
        .await
        .map_err(|_| ErrorBadRequest("invalid image url"))?;

    // 使用 image engine 处理
    // 需要 use crate::engine::Engine; 引入 trait
    let mut engine: Photon = data
        .try_into()
        .map_err(|_| ErrorInternalServerError("deal image failed"))?;
    engine.apply(&spec.specs);
    // TODO: 这里目前类型写死了，应该使用 content negotiation
    let image = engine.generate(ImageOutputFormat::Jpeg(85));

    info!("Finished processing: image size {}", image.len());

    Ok(HttpResponse::Ok().content_type("image/jpeg").body(image))
    // .body(format!("url: {}\n spec: {:#?}", url, spec)))
}

#[instrument(level = "info", skip(cache))]
async fn retrieve_image(url: &str, cache: Cache) -> anyhow::Result<Bytes> {
    let mut hasher = DefaultHasher::new();
    url.hash(&mut hasher);
    let key = hasher.finish();

    // 例子使用的 await 有问题
    let g = &mut cache.lock().unwrap();
    let data = match g.get(&key) {
        Some(v) => {
            info!("Match cache {}", key);
            // @TODO info 没有起作用。使用 print; RUST_LOG=info
            // println!("Match cache {}", key);
            v.to_owned()
        }
        None => {
            info!("Retrieve url");
            let resp = reqwest::get(url).await?;
            let data = resp.bytes().await?;
            g.put(key, data.clone());
            data
        }
    };

    Ok(data)
}

fn print_test_url(url: &str) {
    use std::borrow::Borrow;
    let spec1 = Spec::new_resize(500, 800, resize::SampleFilter::CatmullRom);
    let spec2 = Spec::new_watermark(20, 20);
    let spec3 = Spec::new_filter(filter::Filter::Marine);
    let image_spec = ImageSpec::new(vec![spec1, spec2, spec3]);

    // 为什么要使用 borrow
    let s: String = image_spec.borrow().into();
    let test_image = percent_encode(url.as_bytes(), NON_ALPHANUMERIC).to_string();
    println!("test url: http://localhost:8080/image/{}/{}", s, test_image);
}
