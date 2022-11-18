mod app;
use ajax::Ajax;
use app::build_app;
use once_cell::sync::OnceCell;

static AJAX_INSTANCE: OnceCell<Ajax> = OnceCell::new();

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();
    let app = build_app();
    let matches = app.get_matches();
    AJAX_INSTANCE.get_or_init(|| Ajax::new());

    // let url = matches.get_one::<String>("url");
    // if url.is_some() {
    //     if let Err(err) = xx().await {
    //         eprintln!("{}", err);
    //         std::process::exit(1);
    //     }
    //     return Ok(());
    // }
    Ok(())
}
