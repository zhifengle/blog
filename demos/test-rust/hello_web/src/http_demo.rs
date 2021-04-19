use reqwest::header;

// 在 main 里面能够正常使用，但是不知道为什么单独在一个函数里面不行
pub async fn fetch_data() -> Result<(), Box<dyn std::error::Error>> {
    //let url = "https://httpbin.org/ip";
    let url = "https://www.google.com/";
    let proxy = reqwest::Proxy::all("http://127.0.0.1:1087")?;
    // 自定义头
    let mut headers = header::HeaderMap::new();
    headers.insert("X-MY-HEADER", header::HeaderValue::from_static("value"));
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; WOW64; rv:56.0) Gecko/20100101 Firefox/56.0")
        .proxy(proxy)
        .build()?;
    let resp = client
        .get(url)
        .send()
        .await?
        .text()
        //.text_with_charset("utf-8")
        .await?;
    println!("{}", resp);
    Ok(())
}
