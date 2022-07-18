use std::{error::Error, path::Path};

use clap::{arg, Command};

type MyResult<T> = Result<T, Box<dyn Error>>;

fn main() {
    if let Err(err) = run() {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}

// 更多的例子 https://github.com/clap-rs/clap/tree/master/examples

// 这个能校验路径
// value_parser(clap::value_parser!(std::path::PathBuf))

fn run() -> MyResult<()> {
    // let matches = build_app().get_matches_from(env::args_os());
    // let matches = build_app().get_matches();
    let matches = build_app().get_matches_from(vec![
        "my-cli",
        "my_folder",
        // "--chrome",
        "--firefox-config",
        "my_firefox",
        "-e",
        "f1",
        "-e",
        "f2",
    ]);
    println!("{}", matches.get_one::<String>("target_dir").unwrap());
    // -o out; -o 不能当标识
    println!("{}", matches.get_one::<String>("output_dir").unwrap());

    // bool
    println!("{}", matches.is_present("chrome"));
    println!("{}", matches.contains_id("chrome"));

    // 单个设置; value_of 废弃
    println!("{}", matches.value_of("firefox").unwrap());
    println!("{}", matches.get_one::<String>("firefox").unwrap());

    // 多个
    let exclude_dirs: Vec<&str> = matches.values_of("exclude").unwrap().collect();
    // get_many; 方便配合 value_parser ?
    let exclude_dirs: Vec<_> = matches
        .get_many::<String>("exclude")
        .unwrap()
        .map(|s| s.as_str())
        .collect();
    println!("{:?}", exclude_dirs);

    set_working_dir(&matches)?;

    Ok(())
}

fn build_app() -> Command<'static> {
    let app = Command::new("my-cli")
        .version("0.0.1")
        .about("clap usage")
        // <target_dir> 是 group id
        .arg(arg!(<target_dir> "target dir"))
        .arg(arg!(-o [output_dir] "output dir").default_value("-"))
        // 多个 -e f1 -e f2
        .arg(arg!(-e --exclude ... "exclude dir").default_value("done"))
        // bool; 省略 `chrome:`
        // .arg(arg!(chrome: -c --chrome "use chrome").conflicts_with("firefox"))
        .arg(arg!(-c --chrome "use chrome").conflicts_with("firefox"))
        // 如果不指定名字 , `firefox-config` 会当成group id
        .arg(arg!(firefox: -f --"firefox-config" [firefox] "use firefox").conflicts_with("chrome"));

    app
}

// 使用 anyhow 的 result
fn set_working_dir(_matches: &clap::ArgMatches) -> MyResult<()> {
    let _current_directory = Path::new(".");
    Ok(())
}
