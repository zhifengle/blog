use std::{path::PathBuf, time::Duration};

// 使用 default trait 初始一个对象
#[derive(Debug, Default, PartialEq)]
struct MyConfiguration {
    output: Option<PathBuf>,
    // 默认空 Vec
    search_path: Vec<PathBuf>,
    timeout: Duration,
    // 默认 false
    check: bool,
}

impl MyConfiguration {}

// https://rust-unofficial.github.io/patterns/idioms/default.html
fn the_default_trait() {
    // 全部初始为默认值了
    let mut conf = MyConfiguration::default();
    conf.check = true;

    // Debug print
    println!("conf = {:#?}", conf);

    let conf1 = MyConfiguration {
        check: true,
        ..Default::default()
    };
    assert_eq!(conf, conf1);
}
