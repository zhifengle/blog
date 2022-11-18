use serde::{Deserialize, Serialize};
use serde_json::json;

// https://github.com/seanmonstar/reqwest/blob/master/examples/simple.rs
// `tokio = { version = "1", features = ["full"] }`
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // echo_get().await?;
    // echo_json().await?;
    echo_typed_json().await?;
    Ok(())
}

fn create_client() -> reqwest::Client {
    let ua = String::from("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36");
    let proxy_url = "http://127.0.0.1:10809";
    let proxy = reqwest::Proxy::all(proxy_url).unwrap();

    let client = reqwest::Client::builder()
        .user_agent(ua)
        .proxy(proxy)
        .build()
        .unwrap();

    client
}

async fn echo_get() -> Result<(), reqwest::Error> {
    let url = "http://httpbin.org/get";
    // 推荐使用 builder 创建 Client 实例;
    // let res = reqwest::get(url).await?;
    let client = create_client();
    let res = client.get(url).send().await?;

    eprintln!("Response: {:?} {}", res.version(), res.status());
    eprintln!("Headers: {:#?}\n", res.headers());

    let body = res.text().await?;

    println!("{}", body);

    Ok(())
}

async fn echo_json() -> Result<(), reqwest::Error> {
    let client = create_client();
    let echo_json: serde_json::Value = client
        .post("http://httpbin.org/post")
        .json(&serde_json::json!({
            "title": "Reqwest.rs",
            "body": "https://docs.rs/reqwest",
            "userId": 1
        }))
        .send()
        .await?
        .json()
        .await?;
    println!("{:#?}", echo_json);
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct Form {
    id: i32,
    title: String,
}

async fn echo_typed_json() -> Result<(), Box<dyn std::error::Error>> {
    // let form = Form {
    //     id: 22,
    //     title: "xxx".into(),
    // };
    // impl From<i32> for Value
    let n: serde_json::Value = 8.into();
    let form = json!({
        "id": n,
        "title": "xxx"
    });

    let client = create_client();
    let mut echo_json: serde_json::Value = client
        .post("http://httpbin.org/post")
        .json(&form)
        .send()
        .await?
        .json()
        .await?;

    // 修改 echo_json 取走 `json` child
    let new_form: Form = serde_json::from_value(echo_json["json"].take())?;

    print!("{:#?}", new_form);
    Ok(())
}
