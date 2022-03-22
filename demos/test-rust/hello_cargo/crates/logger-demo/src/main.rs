fn main() {
    tracing_subscriber::fmt::init();

    tracing_demo();
    log::warn!("[root] warn");
    log::info!("[root] info");
    log::debug!("[root] debug");
    foo::run();
}

fn tracing_demo() {
    // Windows: set RUST_LOG=info;
    //  set RUST_LOG=debug,info; 这个尝试无效

    let number_of_yaks = 3;
    // this creates a new event, outside of any spans.
    tracing::info!(number_of_yaks, "preparing to shave yaks");
}

mod foo {
    mod bar {
        pub fn run() {
            tracing::warn!("[bar] warn");
            tracing::info!("[bar] info");
            tracing::debug!("[bar] debug");
        }
    }

    pub fn run() {
        tracing::warn!("[foo] warn");
        tracing::info!("[foo] info");
        tracing::debug!("[foo] debug");
        bar::run();
    }
}

// 测试失败 2022-3-22
// https://rust-lang-nursery.github.io/rust-cookbook/development_tools/debugging/config_log.html
// RUST_LOG="warn,test::foo=info,test::foo::bar=debug"
