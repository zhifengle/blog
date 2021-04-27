use std::env;
use std::process;

use minigrep2::Config;

// main 函数的职责
// 调用命令行的解析逻辑
// 进行一些配置
// 调用 lib.rs 里面的 run
// 处理 run 返回的错误
fn main() {
    let args: Vec<String> = env::args().collect();
    //let config = parse_config(&args);
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    // 分离 main 的逻辑到 run;
    // run 的错误抛给 main？  ---->  Result<(), Box<dyn Error>>
    //run(config);  // 这里没处理 run 抛出来的错误
    if let Err(e) = minigrep2::run(config) {
        eprintln!("Application err: {}", e);

        process::exit(1);
    }
}



// 这里用了 clone;
// args 可以是 &[String]
// 可以改造成 Config::new(xx)
#[allow(dead_code)]
fn parse_config(args: &Vec<String>) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();
    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

    Config { query, filename, case_sensitive }
}
