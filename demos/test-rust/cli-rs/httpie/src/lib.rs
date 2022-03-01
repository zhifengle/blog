use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
// my_string.green()  String 添加颜色
use colored::Colorize;
use mime::Mime;
use reqwest::{header, Client, Response, Url};
use std::{collections::HashMap, str::FromStr};
// 语法高亮。跳过
use syntect::{
    easy::HighlightLines,
    highlighting::{Style, ThemeSet},
    parsing::SyntaxSet,
    util::{as_24_bit_terminal_escaped, LinesWithEndings},
};

// ？？替代 KvPair
// type Params = HashMap<String, String>;

#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "bla")]
pub struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    Get(Get),
    Post(Post),
}

#[derive(Parser, Debug)]
struct Get {
    #[clap(parse(try_from_str=parse_url))]
    url: String,
}
#[derive(Parser, Debug)]
struct Post {
    #[clap(parse(try_from_str=parse_url))]
    url: String,
    #[clap(parse(try_from_str=parse_kv_pair))]
    body: Vec<KvPair>,
}

#[derive(Parser, Debug)]
struct KvPair {
    k: String,
    v: String,
}

// let p = Point::from_str("(1,2)"); 标准库里面的例子
impl FromStr for KvPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("=");
        let err = || anyhow!(format!("Failed to parse {}", s));

        Ok(Self {
            k: (split.next().ok_or_else(err)?).to_string(),
            v: (split.next().ok_or_else(err)?).to_string(),
        })
    }
}

fn parse_kv_pair(s: &str) -> Result<KvPair> {
    Ok(s.parse()?)
}

fn parse_url(s: &str) -> Result<String> {
    let _url: Url = s.parse()?;
    Ok(s.into())
}

async fn get(client: Client, args: &Get) -> Result<()> {
    let res = client.get(&args.url).send().await?;
    print_res(res).await?;
    Ok(())
}

async fn post(client: Client, args: &Post) -> Result<()> {
    let mut body = HashMap::new();
    for pair in args.body.iter() {
        body.insert(&pair.k, &pair.v);
    }
    let res = client.post(&args.url).json(&body).send().await?;
    print_res(res).await?;
    Ok(())
}

async fn print_res(res: Response) -> Result<()> {
    print_status(&res);
    print_headers(&res);
    let mime = get_content_type(&res);
    let body = res.text().await?;
    print_body(mime, &body);
    Ok(())
}

fn print_status(res: &Response) {
    println!(
        "{}\n",
        format!("{:?} {}", res.version(), res.status()).blue()
    );
}

fn print_headers(res: &Response) {
    for (name, value) in res.headers() {
        println!("{}: {:?}", name.to_string().green(), value);
    }
    println!();
}

fn get_content_type(res: &Response) -> Option<Mime> {
    res.headers()
        .get(header::CONTENT_TYPE)
        .map(|v| v.to_str().unwrap().parse().unwrap())
}

// 跳过直接复制的
fn print_body(m: Option<Mime>, body: &str) {
    match m {
        // 对于 "application/json" 我们 pretty print
        Some(v) if v == mime::APPLICATION_JSON => print_syntect(body, "json"),
        Some(v) if v == mime::TEXT_HTML => print_syntect(body, "html"),

        // 其它 mime type，我们就直接输出
        _ => println!("{}", body),
    }
}
// 跳过直接复制的
fn print_syntect(s: &str, ext: &str) {
    // Load these once at the start of your program
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let syntax = ps.find_syntax_by_extension(ext).unwrap();
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
    for line in LinesWithEndings::from(s) {
        let ranges: Vec<(Style, &str)> = h.highlight(line, &ps);
        let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
        print!("{}", escaped);
    }
}

#[tokio::main]
pub async fn run() -> Result<()> {
    // 使用 parse 没有必填参数 exit(2)
    let args = Opts::try_parse()?;
    let mut headers = header::HeaderMap::new();
    headers.insert("X-POWER-BY", "Rust".parse()?);
    // header::HeaderName::from_str(key).unwrap();  // 这种方法也能生成
    // headers.insert(header::USER_AGENT, "Rust Httpie".parse()?);
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;
    match args.subcmd {
        SubCommand::Get(ref get_args) => get(client, get_args).await?,
        SubCommand::Post(ref post_args) => post(client, post_args).await?,
    }
    Ok(())
}
