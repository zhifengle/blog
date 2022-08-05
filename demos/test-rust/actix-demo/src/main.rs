use std::{
    fmt,
    process::{Command, Stdio},
    thread,
};

use actix_web::{
    http::StatusCode, post, web, App, HttpResponse, HttpServer, Responder, ResponseError, Result,
};
use serde_json::json;

#[derive(serde::Deserialize)]
struct Cmd {
    name: String,
    args: Vec<String>,
}

#[derive(serde::Serialize)]
struct Success {
    code: u16,
    data: serde_json::Value,
}

#[derive(Debug, serde::Serialize)]
struct WebError {
    msg: String,
    status: u16,
}

impl fmt::Display for WebError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())
    }
}

impl ResponseError for WebError {
    // builds the actual response to send back when an error occurs
    fn error_response(&self) -> HttpResponse {
        let err_json = json!({ "error": self.msg });
        HttpResponse::build(StatusCode::from_u16(self.status).unwrap()).json(err_json)
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = std::env::var("PORT").unwrap_or("8080".to_string());
    let port = port.parse().unwrap();
    HttpServer::new(|| App::new().service(echo).service(run_cmd))
        .bind(("127.0.0.1", port))?
        .run()
        .await
}

#[post("/v1/run")]
async fn run_cmd(cmd: web::Json<Cmd>) -> Result<impl Responder> {
    let p = get_cmd_path(&cmd.name)?;
    if p.is_empty() {
        return Err(WebError {
            msg: "invalid name".to_string(),
            status: 400,
        }
        .into());
    }
    thread::spawn(move || {
        let _output = Command::new(&p)
            .args(&cmd.args)
            .stdout(Stdio::piped())
            .output()
            .expect("failed to execute process");
        // println!("{}", String::from_utf8_lossy(&output.stdout))
    });
    Ok(web::Json(Success {
        code: 200,
        data: json!(""),
    }))
}

#[post("/v1/cmd")]
async fn echo(cmd: web::Json<Cmd>) -> Result<impl Responder> {
    let p = get_cmd_path(&cmd.name)?;
    if p.is_empty() {
        return Err(WebError {
            msg: "invalid name".to_string(),
            status: 400,
        }
        .into());
    }
    let output = Command::new(&p)
        .args(&cmd.args)
        .stdout(Stdio::piped())
        .output()?;
    // .expect("failed to execute process");
    let code = output.status.code();
    if code.is_none() || code != Some(0) {
        let s = String::from_utf8_lossy(&output.stderr).to_string();
        return Ok(web::Json(Success {
            code: 500,
            data: json!(s),
        }));
    }
    let s = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(web::Json(Success {
        code: 200,
        data: json!(s),
    }))
}

fn get_cmd_path(name: &str) -> Result<String> {
    let contents = std::fs::read_to_string("config.json")?;
    let v: serde_json::Value = serde_json::from_str(&contents)?;
    let path = &v[name];
    if path.is_string() {
        return Ok(path.as_str().unwrap().to_string());
    }

    Ok("".to_string())
}
